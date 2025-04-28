use serde::{Deserialize, Serialize};

use super::state::{Pattern, Track};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerAction {
    SetProjectName { string: String },

    CommChangeAddr { string: String },

    TrackAdd { name: String },
    TrackDelete { name: String },
    TrackEdit { name: String, track: Track },

    PatternAdd { name: String },
    PatternDelete { name: String },
    PatternEdit { name: String, pattern: Pattern },

    TickerSetCycle { cycle: Option<usize> },
    TickerSetBpm { bpm: f32 },
    TickerPlay,
    TickerPause,
    TickerStop,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientAction {
    ProjectNameUpdated { string: String },

    CommAddrChanged { string: String },

    TrackAdded { name: String, track: Track },
    TrackDeleted { name: String },
    TrackEdited { name: String, track: Track },

    PatternAdded { name: String, pattern: Pattern },
    PatternDeleted { name: String },
    PatternEdited { name: String, pattern: Pattern },

    TickerCycleUpdated { cycle: Option<usize> },
    TickerBpmUpdated { bpm: f32 },
    TickerTick { index: usize },
    TickerPlaying,
    TickerPaused,
    TickerStopped,
}
