use std::net::SocketAddr;

use action::{ClientAction, ServerAction};
use axum::extract::ws::{Message, WebSocket};
use state::AppState;
use tracing::info;

pub mod action;
pub mod communicator;
pub mod state;
pub mod ticker;

pub async fn handle_connection(mut socket: WebSocket, addr: SocketAddr, state: AppState) {
    info!("Client connected: {}", addr);

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(action) => {
                let action: ServerAction = serde_json::from_str(&action).unwrap();

                match action {
                    ServerAction::TickerPlay => {
                        state.ticker.lock().unwrap().play();
                        // TODO: let every connection the update
                    }
                    ServerAction::TickerPause => {
                        state.ticker.lock().unwrap().pause();
                        // TODO: let every connection the update
                    }
                    ServerAction::TickerStop => {
                        state.ticker.lock().unwrap().stop();
                        // TODO: let every connection the update
                    }
                    ServerAction::TickerSetBpm { bpm } => {
                        state.ticker.lock().unwrap().set_bpm(bpm);
                        // TODO: let every connection the update
                    }
                    ServerAction::TickerSetCycle { cycle } => {
                        state.ticker.lock().unwrap().set_cycle(cycle);
                        // TODO: let every connection the update
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
    }
}
