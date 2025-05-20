use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{Pattern, Track};

#[rustfmt::skip]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ServerCommand {
    SetProjectName { name: String },

    CommChangeAddr { addr: String },

    CtrlChangeContext { context: Option<String> },

    TrackAdd { name: String },
    TrackDelete { name: String },
    TrackEdit { name: String, track: Track },
    TrackMakeActive { name: String, active: bool, force: bool },
    TrackMakeLoop { name: String, r#loop: bool },

    PatternAdd { name: String },
    PatternDelete { name: String },
    PatternEdit { name: String, pattern: Pattern },

    TickerPlay,
    TickerPause,
    TickerStop,
    TickerSetBpm { bpm: f32 },

    RequestTickerBpm,
    RequestTickerPlaying,
    RequestTickerTick,
    RequestProjectName,
    RequestCommAddr,
    RequestCommStatus,
    RequestCtrlContext,
    RequestTrack { name: String },
    RequestAllTracks,
    RequestPattern { name: String },
    RequestAllPatterns,
}

#[rustfmt::skip]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "payload")]
pub enum ClientCommand {
    ProjectNameUpdated { name: String },

    CommAddrChanged { addr: String },
    CommStatusChanged { established: bool },

    CtrlContextChanged { context: Option<String> },

    TrackAdded { name: String, track: Track },
    TrackDeleted { name: String },
    TrackEdited { name: String, track: Track },
    TrackMadeActive { name: String, active: bool },
    TrackMadeLoop { name: String, r#loop: bool },

    PatternAdded { name: String, pattern: Pattern },
    PatternDeleted { name: String },
    PatternEdited { name: String, pattern: Pattern },
    TrackProgressUpdate { name: String, progress: Option<usize> },

    TickerPlaying,
    TickerPaused,
    TickerStopped,
    TickerTick { tick: usize, max: usize },
    TickerBpmUpdated { bpm: f32 },

    ResponseTickerBpm { bpm: f32 },
    ResponseTickerPlaying { playing: bool },
    ResponseTickerTick { tick: isize, max: usize },
    ResponseProjectName { name: String },
    ResponseCommAddr { addr: String },
    ResponseCommStatus { established: bool },
    ResponseCtrlContext { context: Option<String> },
    ResponseTrack { name: String, track: Track },
    ResponseAllTracks { tracks: HashMap<String, Track> },
    ResponsePattern { name: String, pattern: Pattern },
    ResponseAllPatterns { patterns: HashMap<String, Pattern> },

    Notify { severity: Severity, summary: String, detail: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Success,
    Info,
    Warn,
    Error,
    Secondary,
    Contrast,
}
