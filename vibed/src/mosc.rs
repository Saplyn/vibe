use rosc::OscMessage;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MinOscMessage {
    pub path: String,
    pub args: Vec<MinOscArg>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MinOscArg {
    Float(f32),
    String(String),
}

impl From<MinOscMessage> for OscMessage {
    fn from(val: MinOscMessage) -> Self {
        OscMessage {
            addr: val.path,
            args: val
                .args
                .into_iter()
                .map(|arg| match arg {
                    MinOscArg::Float(f) => rosc::OscType::Float(f),
                    MinOscArg::String(s) => rosc::OscType::String(s),
                })
                .collect(),
        }
    }
}
