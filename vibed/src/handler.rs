use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

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
use tracing::info;
use vibe_types::{
    command::{ClientCommand, ServerCommand},
    models::{Pattern, Track},
};

use crate::{
    communicator::{CommunicatorCommand, CommunicatorState},
    controller::{ControllerCommand, ControllerState},
    ticker::{TickerCommand, TickerState},
};

#[derive(Debug, Clone)]
pub struct HandlerState {
    pub name: Arc<RwLock<String>>,

    pub patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<RwLock<HashMap<String, Track>>>,
    pub tick_rx: watch::Receiver<Option<u8>>,

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
        name,
        patterns,
        tracks,
        tick_rx,
        ticker_cmd_tx,
        controller_cmd_tx,
        communicator_cmd_tx,
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
                            let cmd: ServerCommand = serde_json::from_str(&cmd).unwrap();
                            process(
                                cmd,
                                &mut socket,
                                &ticker_cmd_tx,
                                &client_cmd_broadcast_tx,
                                &ticker_state,
                                &controller_state,
                                &communicator_state,
                            ).await;
                        }
                        Message::Close(_) => {
                            break;
                        }
                        _ => {}
                    }
                }
            }
            Ok(cmd) = client_cmd_broadcast_rx.recv() => {
                info!("Sending to client: {:?}", cmd);
                respond(&mut socket, cmd).await.unwrap();
            }
        }
    }
    info!("Client {} disconnected", addr);
}

async fn respond(socket: &mut WebSocket, cmd: ClientCommand) -> Result<(), axum::Error> {
    let cmd = serde_json::to_string(&cmd).unwrap();
    socket.send(Message::Text(cmd.into())).await
}

async fn process(
    cmd: ServerCommand,
    socket: &mut WebSocket,
    ticker_cmd_tx: &mpsc::Sender<TickerCommand>,
    client_cmd_broadcast_tx: &broadcast::Sender<ClientCommand>,
    ticker_state: &TickerState,
    controller_state: &ControllerState,
    communicator_state: &CommunicatorState,
) {
    match cmd {
        ServerCommand::TickerPlay => {
            ticker_cmd_tx
                .send(crate::ticker::TickerCommand::Play)
                .await
                .unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerPlaying)
                .unwrap();
        }
        ServerCommand::TickerPause => {
            ticker_cmd_tx
                .send(crate::ticker::TickerCommand::Pause)
                .await
                .unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerPaused)
                .unwrap();
        }
        ServerCommand::TickerStop => {
            ticker_cmd_tx
                .send(crate::ticker::TickerCommand::Stop)
                .await
                .unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerStopped)
                .unwrap();
        }
        ServerCommand::TickerSetBpm { bpm } => {
            ticker_cmd_tx
                .send(crate::ticker::TickerCommand::SetBPM { bpm })
                .await
                .unwrap();
            client_cmd_broadcast_tx
                .send(ClientCommand::TickerBpmUpdated { bpm })
                .unwrap();
        }
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
        _ => {
            todo!()
        }
    }
}
