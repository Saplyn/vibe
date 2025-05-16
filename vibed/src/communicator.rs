use std::{sync::Arc, time::Duration};

use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    select,
    sync::{RwLock as AsyncRwLock, mpsc},
    time::sleep,
};
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct CommunicatorState {
    pub target_addr: Arc<AsyncRwLock<String>>,
}

#[derive(Debug)]
pub struct CommunicatorArg {
    pub cmd_rx: mpsc::Receiver<CommunicatorCommand>,
}

#[derive(Debug)]
pub enum CommunicatorCommand {
    ChangeTargetAddr { addr: String },
    // SendMessage { msg: () }
}

pub async fn main(state: CommunicatorState, arg: CommunicatorArg) {
    info!("Communicator started");

    let CommunicatorState { target_addr } = state;
    let CommunicatorArg { mut cmd_rx } = arg;

    loop {
        select! {
            Some(CommunicatorCommand::ChangeTargetAddr { addr }) = cmd_rx.recv() => {
                *target_addr.write().await = addr;
            }
            else => {
                match TcpStream::connect(&*target_addr.read().await).await {
                    Ok(stream) => {
                        process(stream, &mut cmd_rx).await;
                    }
                    Err(err) => {
                        warn!("Failed to connect to target: {}", err);
                        sleep(Duration::from_millis(500)).await;
                    }
                }
            }
        }
    }
}

async fn process(
    mut stream: TcpStream,
    cmd_rx: &mut mpsc::Receiver<CommunicatorCommand>,
) -> Option<String> {
    loop {
        select! {
            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    CommunicatorCommand::ChangeTargetAddr { addr } => {
                        break Some(addr);
                    }
                }
            }
            else => {
                let _ = stream.write_all(b"uwu;").await;
            }
        }
    }
}
