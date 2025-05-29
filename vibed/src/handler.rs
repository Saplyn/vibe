use std::net::SocketAddr;

use axum::{
    extract::{
        ConnectInfo, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use tokio::{
    select,
    sync::{broadcast, mpsc, watch},
};
use tracing::{info, warn};

use crate::{
    command::{ClientCommand, ServerCommand, Severity},
    communicator::{CommunicatorCommand, CommunicatorState},
    controller::{ControllerCommand, ControllerState},
    models::{Event, Pattern, Slider, Track},
    mosc::{MinOscArg, MinOscMessage},
    store::Store,
    ticker::{TickerCommand, TickerState},
};

#[derive(Debug, Clone)]
pub struct HandlerState {
    pub store: Store,

    pub tick_rx: watch::Receiver<(Option<usize>, usize)>,
    pub connection_status_rx: watch::Receiver<bool>,

    pub ticker_cmd_tx: mpsc::Sender<TickerCommand>,
    pub controller_cmd_tx: mpsc::Sender<ControllerCommand>,
    pub communicator_cmd_tx: mpsc::Sender<CommunicatorCommand>,
    pub client_cmd_broadcast: broadcast::Sender<ClientCommand>,

    pub ticker_state: TickerState,
    pub controller_state: ControllerState,
    pub communicator_state: CommunicatorState,
}

pub async fn ws_upgrader(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<HandlerState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| ws_handler(socket, addr, state.0))
}

async fn ws_handler(mut socket: WebSocket, addr: SocketAddr, state: HandlerState) {
    info!("Client connected: {}", addr);

    let HandlerState {
        store,
        mut tick_rx,
        mut connection_status_rx,
        ticker_cmd_tx,
        communicator_cmd_tx,
        controller_cmd_tx,
        client_cmd_broadcast,
        ticker_state,
        controller_state,
        communicator_state,
    } = state;

    let client_cmd_broadcast_tx = client_cmd_broadcast.clone();
    let mut client_cmd_broadcast_rx = client_cmd_broadcast.subscribe();

    loop {
        select! {
            msg = socket.recv() => {
                let Some(msg) = msg else {
                    break;
                };
                if let Ok(msg) = msg {
                    match msg {
                        Message::Text(cmd) => {
                            let Ok(cmd) = serde_json::from_str::<ServerCommand>(&cmd) else {
                                warn!("Failed to parse command: {:?}", cmd);
                                continue;
                            };
                            info!("get messages: {:?}", cmd);
                            process(ProcessArg {
                                cmd,
                                socket: &mut socket,
                                store: store.clone(),
                                tick_rx: &tick_rx,
                                ticker_cmd_tx: &ticker_cmd_tx,
                                controller_cmd_tx: &controller_cmd_tx,
                                communicator_cmd_tx: &communicator_cmd_tx,
                                client_cmd_broadcast_tx: &client_cmd_broadcast_tx,
                                ticker_state: &ticker_state,
                                controller_state: &controller_state,
                                communicator_state: &communicator_state,
                            }).await;
                        }
                        Message::Close(_) => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Ok(cmd) = client_cmd_broadcast_rx.recv() => {
                respond(&mut socket, cmd).await;
            }
            Ok(()) = tick_rx.changed() => {
                let maybe_tick = *tick_rx.borrow_and_update();
                if let (Some(tick), max) = maybe_tick {
                    respond(&mut socket, ClientCommand::TickerTick { tick, max }).await;
                }
            }
            Ok(()) = connection_status_rx.changed() => {
                let established = *connection_status_rx.borrow_and_update();
                respond(&mut socket, ClientCommand::CommStatusChanged {
                    established
                }).await;
            }


        }
    }
    info!("Client {} disconnected", addr);
}

fn broadcast(client_cmd_broadcast_tx: &broadcast::Sender<ClientCommand>, cmd: ClientCommand) {
    if let Err(err) = client_cmd_broadcast_tx.send(cmd) {
        warn!("Failed to broadcast client command: {}", err);
    };
}

async fn respond(socket: &mut WebSocket, cmd: ClientCommand) {
    info!("Sending to client: {:?}", cmd);
    let cmd = serde_json::to_string(&cmd).expect("Failed to serialize command");
    if let Err(err) = socket.send(Message::Text(cmd.into())).await {
        warn!("Failed to respond to client: {}", err);
    }
}

#[derive(Debug)]
pub struct ProcessArg<'a> {
    cmd: ServerCommand,
    pub store: Store,
    socket: &'a mut WebSocket,
    tick_rx: &'a watch::Receiver<(Option<usize>, usize)>,
    ticker_cmd_tx: &'a mpsc::Sender<TickerCommand>,
    controller_cmd_tx: &'a mpsc::Sender<ControllerCommand>,
    communicator_cmd_tx: &'a mpsc::Sender<CommunicatorCommand>,
    client_cmd_broadcast_tx: &'a broadcast::Sender<ClientCommand>,
    ticker_state: &'a TickerState,
    controller_state: &'a ControllerState,
    communicator_state: &'a CommunicatorState,
}

async fn process(arg: ProcessArg<'_>) {
    let ProcessArg {
        cmd,
        store,
        socket,
        tick_rx,
        ticker_cmd_tx,
        controller_cmd_tx,
        communicator_cmd_tx,
        client_cmd_broadcast_tx,
        ticker_state,
        controller_state,
        communicator_state,
    } = arg;

    match cmd {
        // LYN: Misc
        ServerCommand::SetProjectName { name: new_name } => {
            *store.name.write().await = new_name.clone();
            broadcast(
                client_cmd_broadcast_tx,
                ClientCommand::ProjectNameUpdated { name: new_name },
            );
        }
        ServerCommand::CommChangeAddr { addr: new_addr } => {
            communicator_cmd_tx
                .send(CommunicatorCommand::ChangeTargetAddr {
                    addr: new_addr.clone(),
                })
                .await
                .unwrap();
            broadcast(
                client_cmd_broadcast_tx,
                ClientCommand::CommAddrChanged { addr: new_addr },
            );
        }
        ServerCommand::CtrlChangeContext { context } => {
            if let Some(context) = context {
                // Context: pattern
                let cycle = store
                    .patterns
                    .read()
                    .await
                    .get(&context)
                    .map(|pat| pat.page_count);
                if cycle.is_some() {
                    controller_cmd_tx
                        .send(ControllerCommand::ChangeContext {
                            context: Some(context.clone()),
                        })
                        .await
                        .unwrap();
                    broadcast(
                        client_cmd_broadcast_tx,
                        ClientCommand::CtrlContextChanged {
                            context: Some(context),
                        },
                    );
                } else {
                    // non-exist pattern
                    respond(
                        socket,
                        ClientCommand::Notify {
                            severity: Severity::Error,
                            summary: "Failed to Change Context".to_string(),
                            detail: format!("Pattern with name \"{}\" does not exist", context),
                        },
                    )
                    .await;
                }
            } else {
                // Context: track
                controller_cmd_tx
                    .send(ControllerCommand::ChangeContext { context: None })
                    .await
                    .unwrap();
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::CtrlContextChanged { context: None },
                );
            }
        }
        // LYN: Pattern
        ServerCommand::PatternAdd { name } => {
            let mut patterns = store.patterns.write().await;
            if patterns.get(&name).is_some() {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Add Pattern {}".to_string(),
                        detail: format!("Pattern with name \"{}\" already exists", name),
                    },
                )
                .await;
            } else {
                let pattern = Pattern::new(name.clone());
                patterns.insert(name.clone(), pattern.clone());
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::PatternAdded { name, pattern },
                );
            }
        }
        ServerCommand::PatternDelete { name } => {
            let mut patterns = store.patterns.write().await;
            if patterns.remove(&name).is_some() {
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::PatternDeleted { name },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Pattern".to_string(),
                        detail: format!("Pattern with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::PatternEdit { name, pattern } => {
            let mut patterns = store.patterns.write().await;
            if let Some(existing_pattern) = patterns.get_mut(&name) {
                *existing_pattern = pattern.clone();
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::PatternEdited { name, pattern },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: format!("Failed to Edit Pattern {}", name),
                        detail: format!("Pattern with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        // LYN: Track
        ServerCommand::TrackAdd { name } => {
            let mut tracks = store.tracks.write().await;
            if tracks.get(&name).is_some() {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Add Track".to_string(),
                        detail: format!("Track with name \"{}\" already exists", name),
                    },
                )
                .await;
            } else {
                let track = Track::new(name.clone());
                tracks.insert(name.clone(), track.clone());
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::TrackAdded { name, track },
                );
            }
        }
        ServerCommand::TrackDelete { name } => {
            let mut tracks = store.tracks.write().await;
            if tracks.remove(&name).is_some() {
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::TrackDeleted { name },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Track".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::TrackEdit { name, track } => {
            let mut tracks = store.tracks.write().await;
            if let Some(existing_track) = tracks.get_mut(&name) {
                *existing_track = track.clone();
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::TrackEdited { name, track },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Edit Track".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::TrackMakeActive {
            name,
            active,
            force,
        } => {
            let mut tracks = store.tracks.write().await;
            if let Some(track) = tracks.get_mut(&name) {
                track.active = active;
                if force {
                    if active {
                        let (tick, _) = *tick_rx.borrow();
                        track.progress = tick.map(|val| val % 4);
                    } else {
                        track.progress = None;
                    }
                }
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::TrackMadeActive {
                        name: name.clone(),
                        active,
                    },
                );
                if let Err(err) = client_cmd_broadcast_tx.send(ClientCommand::TrackProgressUpdate {
                    name: track.name.clone(),
                    progress: track.progress,
                }) {
                    warn!("Failed to broadcast client command: {}", err);
                };
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Make Active".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::TrackMakeLoop { name, r#loop } => {
            let mut tracks = store.tracks.write().await;
            if let Some(track) = tracks.get_mut(&name) {
                track.r#loop = r#loop;
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::TrackMadeLoop { name, r#loop },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Make Loop".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        // LYN: Event
        ServerCommand::EventAdd { name } => {
            let mut events = store.events.write().await;
            if events.get(&name).is_some() {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Add Event".to_string(),
                        detail: format!("Event with name \"{}\" already exists", name),
                    },
                )
                .await;
            } else {
                let event = Event::new(name.clone());
                events.insert(name.clone(), event.clone());
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::EventAdded { name, event },
                );
            }
        }
        ServerCommand::EventDelete { name } => {
            let mut events = store.events.write().await;
            if events.remove(&name).is_some() {
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::EventDeleted { name },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Event".to_string(),
                        detail: format!("Event with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::EventEdit { name, event } => {
            let mut events = store.events.write().await;
            if let Some(existing_event) = events.get_mut(&name) {
                *existing_event = event.clone();
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::EventEdited { name, event },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Edit Event".to_string(),
                        detail: format!("Event with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::EventFire { name } => {
            let events = store.events.read().await;
            let Some(event) = events.get(&name) else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Fire Event".to_string(),
                        detail: format!("Event with name \"{}\" does not exist", name),
                    },
                )
                .await;
                return;
            };
            communicator_cmd_tx
                .send(CommunicatorCommand::SendMessage {
                    msg: MinOscMessage {
                        path: event.path.clone(),
                        arg: event.payload.clone(),
                    },
                })
                .await
                .unwrap();
        }
        // LYN: Slider
        ServerCommand::SliderAdd { name } => {
            let mut sliders = store.sliders.write().await;
            if sliders.get(&name).is_some() {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Add Slider".to_string(),
                        detail: format!("Slider with name \"{}\" already exists", name),
                    },
                )
                .await;
            } else {
                let slider = Slider::new(name.clone());
                sliders.insert(name.clone(), slider.clone());
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::SliderAdded { name, slider },
                );
            }
        }
        ServerCommand::SliderDelete { name } => {
            let mut sliders = store.sliders.write().await;
            if sliders.remove(&name).is_some() {
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::SliderDeleted { name },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Slider".to_string(),
                        detail: format!("Slider with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::SliderEdit { name, slider } => {
            let mut sliders = store.sliders.write().await;
            if let Some(existing_slider) = sliders.get_mut(&name) {
                *existing_slider = slider.clone();
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::SliderEdited { name, slider },
                );
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Edit Slider".to_string(),
                        detail: format!("Slider with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        ServerCommand::SliderSetVal { name, val } => {
            let mut sliders = store.sliders.write().await;
            if let Some(slider) = sliders.get_mut(&name) {
                slider.val = val;
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::SliderValSet { name, val },
                );
                communicator_cmd_tx
                    .send(CommunicatorCommand::SendMessage {
                        msg: MinOscMessage {
                            path: slider.path.clone(),
                            arg: MinOscArg::Float(val),
                        },
                    })
                    .await
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Set Slider Value".to_string(),
                        detail: format!("Slider with name \"{}\" does not exist", name),
                    },
                )
                .await;
            }
        }
        // LYN: Ticker
        ServerCommand::TickerPlay => {
            ticker_cmd_tx.send(TickerCommand::Play).await.unwrap();
            broadcast(client_cmd_broadcast_tx, ClientCommand::TickerPlaying);
        }
        ServerCommand::TickerPause => {
            ticker_cmd_tx.send(TickerCommand::Pause).await.unwrap();
            broadcast(client_cmd_broadcast_tx, ClientCommand::TickerPaused);
        }
        ServerCommand::TickerStop => {
            ticker_cmd_tx.send(TickerCommand::Stop).await.unwrap();
            broadcast(client_cmd_broadcast_tx, ClientCommand::TickerStopped);
            for track in store.tracks.write().await.values_mut() {
                track.progress = None;
                broadcast(
                    client_cmd_broadcast_tx,
                    ClientCommand::TrackProgressUpdate {
                        name: track.name.clone(),
                        progress: None,
                    },
                );
            }
        }
        ServerCommand::TickerSetBpm { bpm } => {
            ticker_cmd_tx
                .send(TickerCommand::SetBPM { bpm })
                .await
                .unwrap();
            broadcast(
                client_cmd_broadcast_tx,
                ClientCommand::TickerBpmUpdated { bpm },
            );
        }
        // LYN: Request
        ServerCommand::RequestTickerBpm => {
            respond(
                socket,
                ClientCommand::ResponseTickerBpm {
                    bpm: *ticker_state.bpm.read().await,
                },
            )
            .await;
        }
        ServerCommand::RequestTickerPlaying => {
            respond(
                socket,
                ClientCommand::ResponseTickerPlaying {
                    playing: *ticker_state.playing.read().await,
                },
            )
            .await;
        }
        ServerCommand::RequestTickerTick => {
            let (tick, max) = *tick_rx.borrow();
            respond(
                socket,
                ClientCommand::ResponseTickerTick {
                    tick: tick.map(|val| val as isize).unwrap_or(-1),
                    max,
                },
            )
            .await;
        }
        ServerCommand::RequestProjectName => {
            respond(
                socket,
                ClientCommand::ResponseProjectName {
                    name: store.name.read().await.clone(),
                },
            )
            .await;
        }
        ServerCommand::RequestCommAddr => {
            respond(
                socket,
                ClientCommand::ResponseCommAddr {
                    addr: communicator_state.target_addr.read().await.clone(),
                },
            )
            .await;
        }
        ServerCommand::RequestCommStatus => {
            respond(
                socket,
                ClientCommand::ResponseCommStatus {
                    established: *communicator_state.connected.read().await,
                },
            )
            .await;
        }
        ServerCommand::RequestAllTracks => {
            respond(
                socket,
                ClientCommand::ResponseAllTracks {
                    tracks: store.tracks.read().await.clone(),
                },
            )
            .await;
        }
        ServerCommand::RequestAllPatterns => {
            respond(
                socket,
                ClientCommand::ResponseAllPatterns {
                    patterns: store.patterns.read().await.clone(),
                },
            )
            .await;
        }
        ServerCommand::RequestCtrlContext => {
            respond(
                socket,
                ClientCommand::ResponseCtrlContext {
                    context: controller_state.context.read().await.clone(),
                },
            )
            .await;
        }
        ServerCommand::RequestAllEvents => {
            respond(
                socket,
                ClientCommand::ResponseAllEvents {
                    events: store.events.read().await.clone(),
                },
            )
            .await;
        }
        ServerCommand::RequestAllSliders => {
            respond(
                socket,
                ClientCommand::ResponseAllSliders {
                    sliders: store.sliders.read().await.clone(),
                },
            )
            .await;
        }
    }
}
