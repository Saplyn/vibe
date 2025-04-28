use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{osc::MinOscMessage, ticker::Ticker};

// LYN: State

#[derive(Debug, Default)]
pub struct AppState {
    pub meta: Meta,
    pub patterns: HashMap<String, Pattern>,
    pub tracks: HashMap<String, Track>,
    pub ticker: Ticker,
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

// LYN: Meta

#[derive(Debug)]
pub struct Meta {
    pub name: String,
    pub target_addr: String, // addr & port
}

const DEFAULT_TARGET_ADDR: &str = "127.0.0.1:3000";
const DEFAULT_PROJECY_NAME: &str = "Unnamed";

impl Default for Meta {
    fn default() -> Self {
        Self {
            name: String::from(DEFAULT_PROJECY_NAME),
            target_addr: String::from(DEFAULT_TARGET_ADDR),
        }
    }
}
