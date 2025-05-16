use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{Pattern, Track};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ServerCommand {
    SetProjectName { name: String },

    CommChangeAddr { addr: String },

    CtrlChangeContext { context: Option<String> },

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

    RequestProjectName,
    RequestCommAddr,
    RequestCtrlContext,
    RequestTrack { name: String },
    RequestAllTracks,
    RequestPattern { name: String },
    RequestAllPatterns,
    RequestTickerBpm,
    RequestTickerPlaying,
    RequestTickerTick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ClientCommand {
    ProjectNameUpdated { name: String },

    CommAddrChanged { addr: String },

    CtrlContextChanged { context: Option<String> },

    TrackAdded { name: String, track: Track },
    TrackDeleted { name: String },
    TrackEdited { name: String, track: Track },

    PatternAdded { name: String, pattern: Pattern },
    PatternDeleted { name: String },
    PatternEdited { name: String, pattern: Pattern },

    TickerBpmUpdated { bpm: f32 },
    TickerTick { tick: u8 },
    TickerPlaying,
    TickerPaused,
    TickerStopped,

    ResponseProjectName { name: String },
    ResponseCommAddr { addr: String },
    ResponseCtrlContext { context: Option<String> },
    ResponseTrack { name: String, track: Track },
    ResponseAllTracks { tracks: HashMap<String, Track> },
    ResponsePattern { name: String, pattern: Pattern },
    ResponseAllPatterns { patterns: HashMap<String, Pattern> },
    ResponseTickerBpm { bpm: f32 },
    ResponseTickerPlaying { playing: bool },
    ResponseTickerTick { tick: u8 },
}
