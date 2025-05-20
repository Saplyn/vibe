use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock as AsyncRwLock;

use crate::models::{Pattern, Track};

#[derive(Debug, Clone)]
pub struct Store {
    pub patterns: Arc<AsyncRwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<AsyncRwLock<HashMap<String, Track>>>,
}

impl Store {
    pub fn load() -> Self {
        Self {
            patterns: Default::default(),
            tracks: Default::default(),
        }
    }

    pub fn save() {}
}
