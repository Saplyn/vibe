use serde::{Deserialize, Serialize};

use crate::mosc::{MinOscArg, MinOscMessage};

// LYN: Page

type Page<T> = [T; 4];

// LYN: Pattern

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub page_count: usize,
    pub midi_path: String,
    pub midi_codes: Vec<Page<Option<u8>>>,
    pub messages: Vec<Messages>,
}

impl Pattern {
    pub fn new(name: String) -> Self {
        Self {
            name,
            page_count: 0,
            midi_path: String::new(),
            midi_codes: Vec::new(),
            messages: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Messages {
    pub payload: MinOscMessage,
    pub actives: Vec<Page<bool>>,
}

impl Pattern {
    pub fn tick_count(&self) -> usize {
        self.page_count * 4
    }
    pub fn resize(&mut self, page_count: usize) {
        self.page_count = page_count;
        self.midi_codes.resize(page_count, [None, None, None, None]);
        for message in &mut self.messages {
            message
                .actives
                .resize(page_count, [false, false, false, false]);
        }
    }
    pub fn get_osc_messages(&self, tick: usize) -> Vec<MinOscMessage> {
        let (page, index) = (tick / 4, tick % 4);
        assert!(
            page <= self.page_count,
            "page {page} <= count {}",
            self.page_count
        );

        let mut ret = Vec::new();
        if let Some(midi_code) = self.midi_codes[page][index] {
            ret.push(MinOscMessage {
                path: self.midi_path.to_owned(),
                arg: MinOscArg::Float(midi_code as f32),
            });
        }
        for message in &self.messages {
            if message.actives[page][index] {
                ret.push(message.payload.to_owned());
            }
        }
        ret
    }
}

// LYN: Track

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub progress: Option<usize>,
    pub patterns: Vec<String>,
}

impl Track {
    pub fn new(name: String) -> Self {
        Self {
            name,
            progress: None,
            patterns: Vec::new(),
        }
    }
}
