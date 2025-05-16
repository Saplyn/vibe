use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tokio::sync::{mpsc, watch};
use tracing::{info, warn};

use vibe_types::models::{Pattern, Track};

#[derive(Debug, Clone)]
pub struct ControllerState {
    pub context: Arc<RwLock<Option<String>>>, // pattern name, empty for tracks
}

#[derive(Debug)]
pub struct ControllerArg {
    pub patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<RwLock<HashMap<String, Track>>>,
    pub cmd_rx: mpsc::Receiver<ControllerCommand>,
    pub tick_rx: watch::Receiver<Option<u8>>,
}

#[derive(Debug)]
pub enum ControllerCommand {
    ChangeContext { context: Option<String> },
}

pub async fn main(state: ControllerState, arg: ControllerArg) {
    info!("Controller started");

    let ControllerState { context } = state;
    let ControllerArg {
        patterns,
        tracks,
        cmd_rx,
        tick_rx,
    } = arg;

    loop {
        if let Some(pattern_name) = context.read().unwrap().as_ref() {
            let patterns = patterns.read().unwrap();
            let Some(pattern) = patterns.get(pattern_name) else {
                warn!("Pattern {} not found", pattern_name);
                // FIXME: wait for new context change?
                continue;
            };
        } else {
            // trace!("Controller track play not impled")
        }
    }

    // TODO:
}
