use std::{collections::HashMap, sync::Arc};

use tokio::{
    select,
    sync::{RwLock as AsyncRwLock, mpsc, watch},
};
use tracing::{info, trace, warn};

use vibe_types::models::{Pattern, Track};

#[derive(Debug, Clone)]
pub struct ControllerState {
    pub context: Arc<AsyncRwLock<Option<String>>>, // pattern name, empty for tracks
}

#[derive(Debug)]
pub struct ControllerArg {
    pub patterns: Arc<AsyncRwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<AsyncRwLock<HashMap<String, Track>>>,
    pub cmd_rx: mpsc::Receiver<ControllerCommand>,
    pub tick_rx: watch::Receiver<Option<usize>>,
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
        mut cmd_rx,
        mut tick_rx,
    } = arg;

    loop {
        select! {
            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    ControllerCommand::ChangeContext { context: new_context } => {
                        *context.write().await = new_context;
                    }
                }
            }
            Ok(()) = tick_rx.changed() => {
                let Some(tick) = *tick_rx.borrow_and_update() else {
                    continue;
                };

                if let Some(pattern_name) = context.read().await.as_ref() {
                    let patterns = patterns.read().await;
                    let Some(pattern) = patterns.get(pattern_name) else {
                        warn!("Pattern {} not found", pattern_name);
                        continue;
                    };
                    pattern.get_osc_messages(tick);
                } else {
                    trace!("Controller track play not impled")
                }
            }

        }
    }

    // TODO:
}
