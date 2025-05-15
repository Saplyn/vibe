use serde::{Deserialize, Serialize};

use crate::state::{Pattern, Track};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ServerCommand {
    SetProjectName { name: String },

    CommChangeAddr { addr: String },
    CommChangeContext { context: Option<String> },

    TrackAdd { name: String },
    TrackDelete { name: String },
    TrackEdit { name: String, track: Track },

    PatternAdd { name: String },
    PatternDelete { name: String },
    PatternEdit { name: String, pattern: Pattern },

    TickerSetBpm { bpm: f32 },
    TickerPlay,
    TickerPause,
    TickerStop,

    RequstProjectName,
    RequestCommAddr,
    RequestTrack { name: String },
    RequestAllTracks,
    RequestPattern { name: String },
    RequestAllPatterns,
    RequstTickerBpm,
    RequestTickerCycle,
    RequestTickerState,
    RequestTickerTick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ClientCommand {
    ProjectNameUpdated { name: String },

    CommAddrChanged { addr: String },
    CommContextChanged { context: Option<String> },

    TrackAdded { name: String, track: Track },
    TrackDeleted { name: String },
    TrackEdited { name: String, track: Track },

    PatternAdded { name: String, pattern: Pattern },
    PatternDeleted { name: String },
    PatternEdited { name: String, pattern: Pattern },

    TickerBpmUpdated { bpm: f32 },
    TickerTick { index: usize },
    TickerPlaying,
    TickerPaused,
    TickerStopped,

    ResponseProjectName { string: String },
    ResponseCommAddr { string: String },
    ResponseTrack { name: String, track: Track },
    ResponseAllTracks { tracks: Vec<(String, Track)> },
    ResponsePattern { name: String, pattern: Pattern },
    ResponseAllPatterns { patterns: Vec<(String, Pattern)> },
    ResponseTickerBpm { bpm: f32 },
    ResponseTickerCycle { cycle: Option<usize> },
    ResponseTickerState { playing: bool },
    ResponseTickerTick { index: usize },
}
