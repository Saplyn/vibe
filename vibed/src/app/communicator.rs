use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::net::TcpStream;
use tokio::spawn;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tracing::{info, warn};

use crate::state::{Pattern, Track};

use super::ticker::Ticker;

const DEFAULT_TARGET_ADDR: &str = "127.0.0.1:3000";

#[derive(Debug)]
pub struct Communicator {
    action_tx: mpsc::Sender<CommunicatorrAction>,
    context: Option<String>, // pattern name, empty for tracks
    target_addr: String,     // addr & port
    handler: JoinHandle<()>,
}

#[derive(Debug)]
struct CommunicatorState {
    patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    tracks: Arc<RwLock<HashMap<String, Track>>>,
    ticker: Arc<RwLock<Ticker>>,
}

#[derive(Debug)]
enum CommunicatorrAction {
    ChangeTargetAddr(String),
    ChangeContext(Option<String>),
}

impl Communicator {
    pub fn new(
        patterns: Arc<RwLock<HashMap<String, Pattern>>>,
        tracks: Arc<RwLock<HashMap<String, Track>>>,
        ticker: Arc<RwLock<Ticker>>,
    ) -> Self {
        let (action_tx, action_rx) = mpsc::channel(8);

        Self {
            action_tx,
            context: None,
            target_addr: String::from(DEFAULT_TARGET_ADDR),
            handler: spawn(communicator_fn(
                String::from(DEFAULT_TARGET_ADDR),
                action_rx,
                CommunicatorState {
                    patterns,
                    tracks,
                    ticker,
                },
            )),
        }
    }
}

// communicate with the target address using TCP/OSC, reconnect if needed
// context `Some`: each tick, send the pattern at index to the target
// context `None`: each tick, loop through all tracks, do the above
async fn communicator_fn(
    target_addr: String,
    mut action_rx: mpsc::Receiver<CommunicatorrAction>,
    state: CommunicatorState,
) {
    let mut stream = connect_to_target(&target_addr).await;
    let mut context: Option<String> = None;
    let CommunicatorState {
        patterns,
        tracks,
        ticker,
    } = state;

    loop {
        if let Some(pattern_name) = &context {
            let patterns = patterns.read().unwrap();
            let Some(pattern) = patterns.get(pattern_name) else {
                warn!("Pattern {} not found", pattern_name);
                // FIXME: wait for new context change?
                continue;
            };
        } else {
        }
    }
}

async fn connect_to_target(target_addr: &str) -> TcpStream {
    loop {
        match TcpStream::connect(target_addr).await {
            Ok(stream) => {
                info!("Connected to target: {}", target_addr);
                break stream;
            }
            Err(e) => {
                warn!("Connection to target failed: {}", e);
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
    }
}
