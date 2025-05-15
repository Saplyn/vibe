use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use tracing::info;

use crate::{
    command::{ClientCommand, ServerCommand},
    state::AppState,
};

pub async fn handle_connection(mut socket: WebSocket, addr: SocketAddr, state: AppState) {
    info!("Client connected: {}", addr);

    let AppState {
        name,
        patterns,
        tracks,
        ticker_cmd_tx,
        communicator_cmd_tx,
        client_cmd_que,
    } = state;

    let client_cmd_que_tx = client_cmd_que.clone();
    let mut client_cmd_que_rx = client_cmd_que.subscribe();

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(cmd) => {
                let cmd: ServerCommand = serde_json::from_str(&cmd).unwrap();

                match cmd {
                    ServerCommand::TickerPlay => {
                        ticker_cmd_tx
                            .send(crate::ticker::TickerCommand::Play)
                            .await
                            .unwrap();
                        client_cmd_que_tx
                            .send(ClientCommand::TickerPlaying)
                            .unwrap();
                    }
                    ServerCommand::TickerPause => {
                        ticker_cmd_tx
                            .send(crate::ticker::TickerCommand::Pause)
                            .await
                            .unwrap();
                        client_cmd_que_tx.send(ClientCommand::TickerPaused).unwrap();
                    }
                    ServerCommand::TickerStop => {
                        ticker_cmd_tx
                            .send(crate::ticker::TickerCommand::Stop)
                            .await
                            .unwrap();
                        client_cmd_que_tx
                            .send(ClientCommand::TickerStopped)
                            .unwrap();
                    }
                    ServerCommand::TickerSetBpm { bpm } => {
                        ticker_cmd_tx
                            .send(crate::ticker::TickerCommand::SetBPM { bpm })
                            .await
                            .unwrap();
                        client_cmd_que_tx
                            .send(ClientCommand::TickerBpmUpdated { bpm })
                            .unwrap();
                    }
                    _ => {
                        todo!()
                    }
                }
            }
            Message::Close(_) => {
                info!("Client {} disconnected", addr);
                break;
            }
            _ => {}
        }

        // FIXME: not tested
        while let Ok(cmd) = client_cmd_que_rx.try_recv() {
            let cmd = serde_json::to_string(&cmd).unwrap();
            socket.send(Message::Text(cmd.into())).await.unwrap();
        }
    }
}
