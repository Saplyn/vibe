use serde::{Deserialize, Serialize};

use crate::mosc::MinOscMessage;

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
