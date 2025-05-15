use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc};

use crate::command::ClientCommand;
use crate::communicator::CommunicatorCommand;
use crate::mosc::MinOscMessage;
use crate::ticker::TickerCommand;

// LYN: App State

#[derive(Debug, Clone)]
pub struct AppState {
    pub name: Arc<RwLock<String>>,

    pub patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<RwLock<HashMap<String, Track>>>,

    pub ticker_cmd_tx: mpsc::Sender<TickerCommand>,
    pub communicator_cmd_tx: mpsc::Sender<CommunicatorCommand>,
    pub client_cmd_que: broadcast::Sender<ClientCommand>,
}

// LYN: Pattern

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub page_count: usize,
    pub midi_path: String,
    pub midi_codes: Vec<Page<Option<u8>>>,
    pub messages: Vec<Page<Messages>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Messages {
    pub payload: MinOscMessage,
    pub active: Vec<Page<bool>>,
}

impl Pattern {
    pub fn tick_count(&self) -> usize {
        self.page_count * 4
    }
}

// LYN: Track

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub active: bool,
    pub patterns: Vec<String>,
}

type Page<T> = [T; 4];
