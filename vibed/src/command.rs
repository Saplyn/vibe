use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{Event, Pattern, Slider, Track};

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

    EventAdd { name: String },
    EventDelete { name: String },
    EventEdit { name: String, event: Event },
    EventFire { name: String },

    SliderAdd { name: String },
    SliderDelete { name: String },
    SliderEdit { name: String, slider: Slider },
    SliderSetVal { name: String, val: f32 },

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
    RequestAllTracks,
    RequestAllPatterns,
    RequestAllEvents,
    RequestAllSliders,
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
    TrackProgressUpdate { name: String, progress: Option<usize> },

    PatternAdded { name: String, pattern: Pattern },
    PatternDeleted { name: String },
    PatternEdited { name: String, pattern: Pattern },

    EventAdded { name: String, event: Event },
    EventDeleted { name: String },
    EventEdited { name: String, event: Event },

    SliderAdded { name: String, slider: Slider },
    SliderDeleted { name: String },
    SliderEdited { name: String, slider: Slider },
    SliderValSet { name: String, val: f32 },

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
    ResponseAllTracks { tracks: HashMap<String, Track> },
    ResponseAllPatterns { patterns: HashMap<String, Pattern> },
    ResponseAllEvents { events: HashMap<String, Event> },
    ResponseAllSliders { sliders: HashMap<String, Slider> },

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
