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
                respond(&mut socket, cmd).await.unwrap();
            }
            Ok(()) = tick_rx.changed() => {
                let maybe_tick = *tick_rx.borrow_and_update();
                if let (Some(tick), max) = maybe_tick {
                    respond(&mut socket, ClientCommand::TickerTick { tick, max })
                        .await
                        .unwrap();
                }
            }
            Ok(()) = connection_status_rx.changed() => {
                let established = *connection_status_rx.borrow_and_update();
                respond(&mut socket, ClientCommand::CommStatusChanged {
                    established
                }).await.unwrap();
            }


        }
    }
    info!("Client {} disconnected", addr);
}

async fn respond(socket: &mut WebSocket, cmd: ClientCommand) -> Result<(), axum::Error> {
    info!("Sending to client: {:?}", cmd);
    let cmd = serde_json::to_string(&cmd).unwrap();
    socket.send(Message::Text(cmd.into())).await
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
            client_cmd_broadcast_tx
                .send(ClientCommand::ProjectNameUpdated { name: new_name })
                .unwrap();
        }
        ServerCommand::CommChangeAddr { addr: new_addr } => {
            communicator_cmd_tx
                .send(CommunicatorCommand::ChangeTargetAddr {
                    addr: new_addr.clone(),
                })
                .await
                .unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::CommAddrChanged { addr: new_addr })
                .unwrap();
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
                    client_cmd_broadcast_tx
                        .send(ClientCommand::CtrlContextChanged {
                            context: Some(context),
                        })
                        .unwrap();
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
                    .await
                    .unwrap();
                }
            } else {
                // Context: track
                controller_cmd_tx
                    .send(ControllerCommand::ChangeContext { context: None })
                    .await
                    .unwrap();
                client_cmd_broadcast_tx
                    .send(ClientCommand::CtrlContextChanged { context: None })
                    .unwrap();
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
                .await
                .unwrap();
            } else {
                let pattern = Pattern::new(name.clone());
                patterns.insert(name.clone(), pattern.clone());
                client_cmd_broadcast_tx
                    .send(ClientCommand::PatternAdded { name, pattern })
                    .unwrap();
            }
        }
        ServerCommand::PatternDelete { name } => {
            let mut patterns = store.patterns.write().await;
            if patterns.remove(&name).is_some() {
                client_cmd_broadcast_tx
                    .send(ClientCommand::PatternDeleted { name })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Pattern".to_string(),
                        detail: format!("Pattern with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
            }
        }
        ServerCommand::PatternEdit { name, pattern } => {
            let mut patterns = store.patterns.write().await;
            if let Some(existing_pattern) = patterns.get_mut(&name) {
                *existing_pattern = pattern.clone();
                client_cmd_broadcast_tx
                    .send(ClientCommand::PatternEdited { name, pattern })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: format!("Failed to Edit Pattern {}", name),
                        detail: format!("Pattern with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
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
                .await
                .unwrap();
            } else {
                let track = Track::new(name.clone());
                tracks.insert(name.clone(), track.clone());
                client_cmd_broadcast_tx
                    .send(ClientCommand::TrackAdded { name, track })
                    .unwrap();
            }
        }
        ServerCommand::TrackDelete { name } => {
            let mut tracks = store.tracks.write().await;
            if tracks.remove(&name).is_some() {
                client_cmd_broadcast_tx
                    .send(ClientCommand::TrackDeleted { name })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Track".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
            }
        }
        ServerCommand::TrackEdit { name, track } => {
            let mut tracks = store.tracks.write().await;
            if let Some(existing_track) = tracks.get_mut(&name) {
                *existing_track = track.clone();
                client_cmd_broadcast_tx
                    .send(ClientCommand::TrackEdited { name, track })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Edit Track".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
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
                    let (tick, _) = *tick_rx.borrow();
                    track.progress = tick.map(|val| val % 4);
                }
                client_cmd_broadcast_tx
                    .send(ClientCommand::TrackMadeActive {
                        name: name.clone(),
                        active,
                    })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Make Active".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
            }
        }
        ServerCommand::TrackMakeLoop { name, r#loop } => {
            let mut tracks = store.tracks.write().await;
            if let Some(track) = tracks.get_mut(&name) {
                track.r#loop = r#loop;
                client_cmd_broadcast_tx
                    .send(ClientCommand::TrackMadeLoop { name, r#loop })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Make Loop".to_string(),
                        detail: format!("Track with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
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
                .await
                .unwrap();
            } else {
                let event = Event::new(name.clone());
                events.insert(name.clone(), event.clone());
                client_cmd_broadcast_tx
                    .send(ClientCommand::EventAdded { name, event })
                    .unwrap();
            }
        }
        ServerCommand::EventDelete { name } => {
            let mut events = store.events.write().await;
            if events.remove(&name).is_some() {
                client_cmd_broadcast_tx
                    .send(ClientCommand::EventDeleted { name })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Event".to_string(),
                        detail: format!("Event with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
            }
        }
        ServerCommand::EventEdit { name, event } => {
            let mut events = store.events.write().await;
            if let Some(existing_event) = events.get_mut(&name) {
                *existing_event = event.clone();
                client_cmd_broadcast_tx
                    .send(ClientCommand::EventEdited { name, event })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Edit Event".to_string(),
                        detail: format!("Event with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
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
                .await
                .unwrap();
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
                .await
                .unwrap();
            } else {
                let slider = Slider::new(name.clone());
                sliders.insert(name.clone(), slider.clone());
                client_cmd_broadcast_tx
                    .send(ClientCommand::SliderAdded { name, slider })
                    .unwrap();
            }
        }
        ServerCommand::SliderDelete { name } => {
            let mut sliders = store.sliders.write().await;
            if sliders.remove(&name).is_some() {
                client_cmd_broadcast_tx
                    .send(ClientCommand::SliderDeleted { name })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Delete Slider".to_string(),
                        detail: format!("Slider with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
            }
        }
        ServerCommand::SliderEdit { name, slider } => {
            let mut sliders = store.sliders.write().await;
            if let Some(existing_slider) = sliders.get_mut(&name) {
                *existing_slider = slider.clone();
                client_cmd_broadcast_tx
                    .send(ClientCommand::SliderEdited { name, slider })
                    .unwrap();
            } else {
                respond(
                    socket,
                    ClientCommand::Notify {
                        severity: Severity::Error,
                        summary: "Failed to Edit Slider".to_string(),
                        detail: format!("Slider with name \"{}\" does not exist", name),
                    },
                )
                .await
                .unwrap();
            }
        }
        ServerCommand::SliderSetVal { name, val } => {
            let mut sliders = store.sliders.write().await;
            if let Some(slider) = sliders.get_mut(&name) {
                slider.val = val;
                client_cmd_broadcast_tx
                    .send(ClientCommand::SliderValSet { name, val })
                    .unwrap();
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
                .await
                .unwrap();
            }
        }
        // LYN: Ticker
        ServerCommand::TickerPlay => {
            ticker_cmd_tx.send(TickerCommand::Play).await.unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerPlaying)
                .unwrap();
        }
        ServerCommand::TickerPause => {
            ticker_cmd_tx.send(TickerCommand::Pause).await.unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerPaused)
                .unwrap();
        }
        ServerCommand::TickerStop => {
            ticker_cmd_tx.send(TickerCommand::Stop).await.unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerStopped)
                .unwrap();
            for track in store.tracks.write().await.values_mut() {
                track.progress = None;
                client_cmd_broadcast_tx
                    .send(ClientCommand::TrackProgressUpdate {
                        name: track.name.clone(),
                        progress: None,
                    })
                    .unwrap();
            }
        }
        ServerCommand::TickerSetBpm { bpm } => {
            ticker_cmd_tx
                .send(TickerCommand::SetBPM { bpm })
                .await
                .unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerBpmUpdated { bpm })
                .unwrap();
        }
        // LYN: Request
        ServerCommand::RequestTickerBpm => {
            respond(
                socket,
                ClientCommand::ResponseTickerBpm {
                    bpm: *ticker_state.bpm.read().await,
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestTickerPlaying => {
            respond(
                socket,
                ClientCommand::ResponseTickerPlaying {
                    playing: *ticker_state.playing.read().await,
                },
            )
            .await
            .unwrap();
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
            .await
            .unwrap();
        }
        ServerCommand::RequestProjectName => {
            respond(
                socket,
                ClientCommand::ResponseProjectName {
                    name: store.name.read().await.clone(),
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestCommAddr => {
            respond(
                socket,
                ClientCommand::ResponseCommAddr {
                    addr: communicator_state.target_addr.read().await.clone(),
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestCommStatus => {
            respond(
                socket,
                ClientCommand::ResponseCommStatus {
                    established: *communicator_state.connected.read().await,
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestAllTracks => {
            respond(
                socket,
                ClientCommand::ResponseAllTracks {
                    tracks: store.tracks.read().await.clone(),
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestAllPatterns => {
            respond(
                socket,
                ClientCommand::ResponseAllPatterns {
                    patterns: store.patterns.read().await.clone(),
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestCtrlContext => {
            respond(
                socket,
                ClientCommand::ResponseCtrlContext {
                    context: controller_state.context.read().await.clone(),
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestAllEvents => {
            respond(
                socket,
                ClientCommand::ResponseAllEvents {
                    events: store.events.read().await.clone(),
                },
            )
            .await
            .unwrap();
        }
        ServerCommand::RequestAllSliders => {
            respond(
                socket,
                ClientCommand::ResponseAllSliders {
                    sliders: store.sliders.read().await.clone(),
                },
            )
            .await
            .unwrap();
        }
    }
}
