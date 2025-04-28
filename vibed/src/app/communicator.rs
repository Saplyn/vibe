use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tokio::spawn;
use tokio::sync::watch;
use tokio::task::JoinHandle;

use super::state::{Pattern, Track};

const DEFAULT_TARGET_ADDR: &str = "127.0.0.1:3000";

#[derive(Debug)]
pub struct Communicator {
    patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    tracks: Arc<RwLock<HashMap<String, Track>>>,
    tick_rx: watch::Receiver<Option<usize>>,

    target_addr: String, // addr & port
    handler: JoinHandle<()>,
}

impl Communicator {
    pub fn new(
        patterns: Arc<RwLock<HashMap<String, Pattern>>>,
        tracks: Arc<RwLock<HashMap<String, Track>>>,
        tick_rx: watch::Receiver<Option<usize>>,
    ) -> Self {
        Self {
            patterns,
            tracks,
            tick_rx,

            target_addr: String::from(DEFAULT_TARGET_ADDR),
            handler: spawn(communicator_fn()),
        }
    }
}

async fn communicator_fn() {}
