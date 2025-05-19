use std::{collections::HashMap, sync::Arc};

use tokio::{
    select,
    sync::{RwLock as AsyncRwLock, mpsc, watch},
};
use tracing::{info, trace, warn};

use crate::{
    communicator::CommunicatorCommand,
    models::{Pattern, Track},
};

#[derive(Debug, Clone)]
pub struct ControllerState {
    pub context: Arc<AsyncRwLock<Option<String>>>, // pattern name, empty for tracks
}

#[derive(Debug)]
pub struct ControllerArg {
    pub patterns: Arc<AsyncRwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<AsyncRwLock<HashMap<String, Track>>>,
    pub cmd_rx: mpsc::Receiver<ControllerCommand>,
    pub tick_rx: watch::Receiver<(Option<usize>, usize)>,
    pub communicator_cmd_tx: mpsc::Sender<CommunicatorCommand>,
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
        communicator_cmd_tx,
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
                let (Some(tick), _) = *tick_rx.borrow_and_update() else {
                    continue;
                };

                if let Some(pattern_name) = context.read().await.as_ref() {
                    let patterns = patterns.read().await;
                    let Some(pattern) = patterns.get(pattern_name) else {
                        warn!("Pattern {} not found", pattern_name);
                        continue;
                    };
                    for msg in pattern.get_osc_messages(tick) {
                        communicator_cmd_tx.send(CommunicatorCommand::SendMessage { msg })
                            .await
                            .unwrap();
                    }
                } else {
                    trace!("Controller track play not impled");
                    let tracks = tracks.read().await;
                    for track in tracks.iter().filter(|(_, v)| v.active) {

                    }
                }
            }

        }
    }

    // TODO:
}
