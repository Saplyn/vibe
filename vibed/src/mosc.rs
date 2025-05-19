use rosc::OscMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MinOscMessage {
    pub path: String,
    pub arg: MinOscArg,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum MinOscArg {
    Float(f32),
    String(String),
}

impl Default for MinOscArg {
    fn default() -> Self {
        MinOscArg::String("/".to_string())
    }
}

impl From<MinOscMessage> for OscMessage {
    fn from(val: MinOscMessage) -> Self {
        OscMessage {
            addr: val.path,
            args: vec![match val.arg {
                MinOscArg::Float(f) => rosc::OscType::Float(f),
                MinOscArg::String(s) => rosc::OscType::String(s),
            }],
        }
    }
}
