use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tokio::sync::{mpsc, watch};
use tracing::info;

use crate::state::{Pattern, Track};

#[derive(Debug)]
pub struct CommunicatorState {
    pub patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<RwLock<HashMap<String, Track>>>,
    pub target_addr: Arc<RwLock<String>>,
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
}
