use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::Duration,
};

use tokio::{
    net::TcpStream,
    sync::{RwLock as AsyncRwLock, mpsc, watch},
    time::sleep,
};
use tracing::{info, warn};

use crate::state::{Pattern, Track};

#[derive(Debug)]
pub struct CommunicatorState {
    pub patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<RwLock<HashMap<String, Track>>>,
    pub target_addr: Arc<AsyncRwLock<String>>,
    pub context: Option<String>, // pattern name, empty for tracks

    pub cmd_rx: mpsc::Receiver<CommunicatorCommand>,
    pub tick_rx: watch::Receiver<Option<bool>>,
}

#[derive(Debug)]
pub enum CommunicatorCommand {
    ChangeTargetAddr { addr: String },
    ChangeContext { pattern_name: Option<String> },
}

pub async fn main(state: CommunicatorState) {
    info!("Communicator started");

    let CommunicatorState {
        patterns,
        tracks,
        target_addr,
        context,
        cmd_rx: mut action_rx,
        tick_rx,
    } = state;

    let target = {
        let target_addr = target_addr.read().await;
        connect_to_target(&target_addr).await
    };

    loop {
        if let Some(pattern_name) = &context {
            let patterns = patterns.read().unwrap();
            let Some(pattern) = patterns.get(pattern_name) else {
                warn!("Pattern {} not found", pattern_name);
                // FIXME: wait for new context change?
                continue;
            };
        } else {
            todo!("Communicator track play not impled")
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
                sleep(Duration::from_millis(500)).await;
            }
        }
    }
}
