use std::{collections::HashMap, fs, path::Path, sync::Arc};

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock as AsyncRwLock;

use crate::{
    DEFAULT_BPM, DEFAULT_NAME, DEFAULT_TARGET_ADDR,
    models::{Pattern, Track},
};

#[derive(Debug, Clone)]
pub struct Store {
    pub name: Arc<AsyncRwLock<String>>,
    pub bpm: Arc<AsyncRwLock<f32>>,
    pub target_addr: Arc<AsyncRwLock<String>>,
    pub patterns: Arc<AsyncRwLock<HashMap<String, Pattern>>>,
    pub tracks: Arc<AsyncRwLock<HashMap<String, Track>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StrippedStore {
    pub name: String,
    pub bpm: f32,
    pub target_addr: String,
    pub patterns: HashMap<String, Pattern>,
    pub tracks: HashMap<String, Track>,
}

impl From<StrippedStore> for Store {
    fn from(val: StrippedStore) -> Self {
        Store {
            name: Arc::new(AsyncRwLock::new(val.name)),
            bpm: Arc::new(AsyncRwLock::new(val.bpm)),
            target_addr: Arc::new(AsyncRwLock::new(val.target_addr)),
            patterns: Arc::new(AsyncRwLock::new(val.patterns)),
            tracks: Arc::new(AsyncRwLock::new(val.tracks)),
        }
    }
}

impl Default for Store {
    fn default() -> Self {
        Self {
            name: Arc::new(AsyncRwLock::new(DEFAULT_NAME.to_string())),
            bpm: Arc::new(AsyncRwLock::new(DEFAULT_BPM)),
            target_addr: Arc::new(AsyncRwLock::new(DEFAULT_TARGET_ADDR.to_string())),
            patterns: Default::default(),
            tracks: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum StoreSaveError {
    FileWriteError,
    JsonForgeError,
}

impl Store {
    pub fn load(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let Ok(file) = fs::read_to_string(path) else {
            return Self::default();
        };
        let Ok(store) = serde_json::from_str::<StrippedStore>(&file) else {
            return Self::default();
        };
        store.into()
    }

    pub async fn save(&self, path: impl AsRef<Path>) -> Result<(), StoreSaveError> {
        let path = path.as_ref();
        let store = self.snapshot().await;
        let Ok(json) = serde_json::to_string_pretty(&store) else {
            return Err(StoreSaveError::JsonForgeError);
        };
        if fs::write(path, json).is_err() {
            return Err(StoreSaveError::FileWriteError);
        }
        Ok(())
    }

    pub async fn snapshot(&self) -> StrippedStore {
        StrippedStore {
            name: self.name.read().await.clone(),
            bpm: *self.bpm.read().await,
            target_addr: self.target_addr.read().await.clone(),
            patterns: self.patterns.read().await.clone(),
            tracks: self.tracks.read().await.clone(),
        }
    }
}
