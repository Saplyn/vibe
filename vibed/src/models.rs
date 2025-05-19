use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use tokio::sync::RwLock as AsyncRwLock;
use tracing::error;

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
        let (page, index) = (tick / 4, tick % 4);
        if page >= self.page_count {
            error!("page {page} >= count {}", self.page_count);
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
        self.page_count * 4
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
        force: bool,
        patterns_map: Arc<AsyncRwLock<HashMap<String, Pattern>>>,
    ) -> Vec<MinOscMessage> {
        if !self.active {
            return vec![];
        }
        if self.progress.is_none() {
            if force || tick % 4 == 0 {
                self.progress = Some(tick % 4);
            } else {
                return vec![];
            }
        }

        let Some(mut progress) = self.progress else {
            unreachable!()
        };
        let patterns_map = patterns_map.read().await;
        let patterns = self
            .patterns
            .iter()
            .filter_map(|name| patterns_map.get(name))
            .collect::<Vec<_>>();
        let pat = patterns.iter().find(|pat| {
            if progress < pat.tick_count() {
                return true;
            }
            progress -= pat.tick_count();
            false
        });

        self.progress = self.progress.map(|val| {
            if val >= patterns.iter().map(|pat| pat.tick_count()).sum() {
                0
            } else {
                val + 1
            }
        });

        if let Some(pat) = pat {
            pat.get_osc_messages(progress)
        } else {
            vec![]
        }
    }
}
