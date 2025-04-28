use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::RwLock;

use axum::extract::ws::{Message, WebSocket};
use serde::{Deserialize, Serialize};
use state::AppState;
use state::Pattern;
use state::Track;
use tracing::info;

pub mod state;

// LYN: Main Loop

pub async fn handle_connection(
    mut socket: WebSocket,
    addr: SocketAddr,
    state: Arc<RwLock<AppState>>,
) {
    info!("Client connected: {}", addr);

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(action) => {
                let action: ServerAction = serde_json::from_str(&action).unwrap();

                match action {
                    ServerAction::TickerPlay => {
                        state.write().unwrap().ticker.play();
                    }
                    ServerAction::TickerPause => {
                        state.write().unwrap().ticker.pause();
                    }
                    ServerAction::TickerStop => {
                        state.write().unwrap().ticker.stop();
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

// LYN: Actions

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerAction {
    MetaSetProjectName { string: String },
    MetaSetTargetAddr { string: String },

    TrackAdd { name: String },
    TrackDelete { name: String },
    TrackEdit { name: String, track: Track },

    PatternAdd { name: String },
    PatternDelete { name: String },
    PatternEdit { name: String, pattern: Pattern },

    TickerSetContext { cycle: Option<usize>, bpm: f32 },
    TickerPlay,
    TickerPause,
    TickerStop,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientAction {
    MetaProjectNameUpdated { string: String },
    MetaTargetUpdated { string: String },

    TrackAdded { name: String, track: Track },
    TrackDeleted { name: String },
    TrackEdited { name: String, track: Track },

    PatternAdded { name: String, pattern: Pattern },
    PatternDeleted { name: String },
    PatternEdited { name: String, pattern: Pattern },

    TickerContextSwitched { cycle: Option<usize>, bpm: f32 },
    TickerTick { index: usize },
}
