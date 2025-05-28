use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use tokio::sync::RwLock as AsyncRwLock;

use crate::mosc::{MinOscArg, MinOscMessage};

// LYN: Page

const PAGE_SIZE: usize = 4;
type Page<T> = [T; PAGE_SIZE];

// LYN: Pattern

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub name: String,
    pub page_count: usize,
    pub midi_path: String,
    pub midi_codes: Vec<Page<Option<u8>>>,
    pub messages: Vec<Messages>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Messages {
    pub payload: MinOscMessage,
    pub actives: Vec<Page<bool>>,
}

impl Pattern {
    pub fn new(name: String) -> Self {
        Self {
            name,
            page_count: 0,
            midi_path: String::from("/"),
            midi_codes: Vec::new(),
            messages: Vec::new(),
        }
    }
    pub fn get_osc_messages(&self, tick: usize) -> Vec<MinOscMessage> {
        let (page, index) = (tick / PAGE_SIZE, tick % PAGE_SIZE);
        if page >= self.page_count {
            return vec![];
        }

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
    fn tick_count(&self) -> usize {
        self.page_count * PAGE_SIZE
    }
}

// LYN: Track

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub name: String,
    pub active: bool,
    pub r#loop: bool,
    pub progress: Option<usize>,
    pub patterns: Vec<String>,
}

fn mod_beat(total_length: usize, beat: usize) -> usize {
    if total_length >= 16 {
        beat % 16
    } else if total_length >= 8 {
        beat % 8
    } else {
        beat % 4
    }
}

impl Track {
    pub fn new(name: String) -> Self {
        Self {
            name,
            active: false,
            r#loop: false,
            progress: None,
            patterns: Vec::new(),
        }
    }
    pub async fn get_osc_messages_and_advance(
        &mut self,
        tick: usize,
        patterns_map: Arc<AsyncRwLock<HashMap<String, Pattern>>>,
    ) -> Vec<MinOscMessage> {
        let patterns_map = patterns_map.read().await;
        let patterns = self
            .patterns
            .iter()
            .filter_map(|name| patterns_map.get(name))
            .collect::<Vec<_>>();
        let total_length = patterns.iter().map(|pat| pat.tick_count()).sum();

        if !self.active {
            if let Some(progress) = self.progress {
                if mod_beat(total_length, progress) == 0 {
                    self.progress = None;
                    return vec![];
                }
            } else {
                return vec![];
            }
        }
        if self.progress.is_none() {
            if mod_beat(total_length, tick) == 0 {
                self.progress = Some(mod_beat(total_length, tick));
            } else {
                return vec![];
            }
        }

        let Some(mut progress) = self.progress else {
            unreachable!()
        };
        let pat = patterns.iter().find(|pat| {
            if progress < pat.tick_count() {
                return true;
            }
            progress -= pat.tick_count();
            false
        });

        self.progress = if self.progress.unwrap() + 1 >= total_length {
            if self.r#loop {
                Some(0)
            } else {
                self.active = false;
                None
            }
        } else {
            self.progress.map(|val| val + 1)
        };

        if let Some(pat) = pat {
            pat.get_osc_messages(progress)
        } else {
            vec![]
        }
    }
}

// LYN: Event

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Event {
    pub name: String,
    pub path: String,
    pub shortcut: Option<String>,
    pub payload: MinOscArg,
}

impl Event {
    pub fn new(name: String) -> Self {
        Self {
            name,
            path: String::from("/"),
            shortcut: None,
            payload: MinOscArg::default(),
        }
    }
}

// LYN: Slider

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slider {
    pub name: String,
    pub path: String,
    pub val: f32,
    pub max: f32,
    pub min: f32,
}

impl Slider {
    pub fn new(name: String) -> Self {
        Self {
            name,
            path: String::from("/"),
            val: 0.0,
            max: 1.0,
            min: 0.0,
        }
    }
}
