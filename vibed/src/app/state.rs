use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use serde::{Deserialize, Serialize};

use crate::mosc::MinOscMessage;

use super::{communicator::Communicator, ticker::Ticker};

// LYN: State

const DEFAULT_NAME: &str = "Unnamed";

#[derive(Debug, Clone)]
pub struct AppState {
    pub name: String,
    pub patterns: Arc<RwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<RwLock<HashMap<String, Track>>>,
    pub ticker: Arc<Mutex<Ticker>>,
    pub communicator: Arc<RwLock<Communicator>>,
}

impl Default for AppState {
    fn default() -> Self {
        let patterns: Arc<RwLock<HashMap<String, Pattern>>> = Default::default();
        let tracks: Arc<RwLock<HashMap<String, Track>>> = Default::default();
        let ticker: Arc<Mutex<Ticker>> = Default::default();

        let communicator = Arc::new(RwLock::new(Communicator::new(
            patterns.clone(),
            tracks.clone(),
            ticker.lock().unwrap().tick_rx(),
        )));

        Self {
            name: String::from(DEFAULT_NAME),
            patterns,
            tracks,
            ticker,
            communicator,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pattern {
    pub page_count: u32,
    pub midi_path: String,
    pub midi_codes: Vec<Option<u8>>,
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    pub active: bool,
    pub patterns: Vec<Pattern>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub payload: MinOscMessage,
    pub active: Vec<bool>,
}
