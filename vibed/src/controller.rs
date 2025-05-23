use std::{sync::Arc, time::Duration};

use tokio::{
    select,
    sync::{RwLock as AsyncRwLock, broadcast, mpsc, watch},
    time::{Instant, interval_at},
};
use tracing::{info, warn};

use crate::{
    DEFAULT_SAVE_PATH, command::ClientCommand, communicator::CommunicatorCommand, store::Store,
};

#[derive(Debug, Clone)]
pub struct ControllerState {
    pub context: Arc<AsyncRwLock<Option<String>>>, // pattern name, empty for tracks
}

#[derive(Debug)]
pub struct ControllerArg {
    pub store: Store,
    pub cmd_rx: mpsc::Receiver<ControllerCommand>,
    pub tick_rx: watch::Receiver<(Option<usize>, usize)>,
    pub communicator_cmd_tx: mpsc::Sender<CommunicatorCommand>,
    pub client_cmd_broadcast_tx: broadcast::Sender<ClientCommand>,
}

#[derive(Debug)]
pub enum ControllerCommand {
    ChangeContext { context: Option<String> },
}

pub async fn main(state: ControllerState, arg: ControllerArg) {
    info!("Controller started");

    let ControllerState { context } = state;
    let ControllerArg {
        store,
        mut cmd_rx,
        mut tick_rx,
        communicator_cmd_tx,
        client_cmd_broadcast_tx,
    } = arg;

    let mut interval = interval_at(Instant::now(), Duration::from_secs(10));

    loop {
        select! {
            _ = interval.tick() => {
                if let Err(e) = store.save(DEFAULT_SAVE_PATH).await {
                    warn!("Failed to save file: {:?}", e);
                }
            }
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
                    let patterns = store.patterns.read().await;
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
                    let mut tracks = store.tracks.write().await;
                    let mut msgs = Vec::new();
                    for (_, track) in tracks.iter_mut().filter(|(_, t)| t.active || t.progress.is_some()) {
                        msgs.push(
                            track
                                .get_osc_messages_and_advance(tick, store.patterns.clone())
                                .await
                        );

                        client_cmd_broadcast_tx.send(ClientCommand::TrackProgressUpdate {
                            name: track.name.clone(),
                            progress: track.progress,
                        }).unwrap();
                        if !track.active {
                            client_cmd_broadcast_tx.send(ClientCommand::TrackMadeActive {
                                name: track.name.clone(),
                                active: false,
                            }).unwrap();
                        }
                    }
                    for msg in msgs.iter().flatten() {
                        communicator_cmd_tx.send(CommunicatorCommand::SendMessage {
                            msg: msg.clone()
                        }).await.unwrap();
                    }
                }
            }
        }
    }
}
