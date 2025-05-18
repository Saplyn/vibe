use std::{io, sync::Arc, time::Duration};

use rosc::{OscPacket, encoder::encode};
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    select, spawn,
    sync::{RwLock as AsyncRwLock, mpsc, watch},
    task::JoinHandle,
    time::sleep,
};
use tracing::{info, warn};
use vibe_types::mosc::MinOscMessage;

#[derive(Debug, Clone)]
pub struct CommunicatorState {
    pub target_addr: Arc<AsyncRwLock<String>>,
    pub connected: Arc<AsyncRwLock<bool>>,
}

#[derive(Debug)]
pub struct CommunicatorArg {
    pub cmd_rx: mpsc::Receiver<CommunicatorCommand>,
    pub connection_status_tx: watch::Sender<bool>,
}

#[derive(Debug)]
pub enum CommunicatorCommand {
    ChangeTargetAddr { addr: String },
    SendMessage { msg: MinOscMessage },
}

pub async fn main(state: CommunicatorState, arg: CommunicatorArg) {
    info!("Communicator started");

    let CommunicatorState {
        target_addr,
        connected,
    } = state;
    let CommunicatorArg {
        mut cmd_rx,
        connection_status_tx,
    } = arg;

    let update_connection_status = async |status: bool| {
        *connected.write().await = status;
        connection_status_tx.send(status).unwrap();
    };

    loop {
        let addr = (*target_addr.read().await).clone();

        select! {
            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    CommunicatorCommand::ChangeTargetAddr { addr } => {
                        *target_addr.write().await = addr;
                    }
                    CommunicatorCommand::SendMessage { msg } => {
                        warn!("Not connected to TCP server, actively ignoring osc message: {:?}", msg);
                    }
                }
            }
            Ok(res) = spawn_connect(addr) => {
                match res {
                    Ok(stream) => {
                        update_connection_status(true).await;
                        if let Some(addr) = process(stream, &mut cmd_rx).await {
                            *target_addr.write().await = addr;
                        };
                        update_connection_status(false).await;
                    }
                    Err(err) => warn!("{:?}", err),
                }
                sleep(Duration::from_millis(200)).await;
            }
        }
    }
}

// HACK: It seems that making tcp connection isn't canceal safe to be used in inside `select!`,
// thus must be wrapped inside a spawn
fn spawn_connect(addr: String) -> JoinHandle<io::Result<TcpStream>> {
    spawn(async move { TcpStream::connect(addr).await })
}

async fn process(
    mut stream: TcpStream,
    cmd_rx: &mut mpsc::Receiver<CommunicatorCommand>,
) -> Option<String> {
    loop {
        select! {
            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    CommunicatorCommand::ChangeTargetAddr { addr } => break Some(addr),
                    CommunicatorCommand::SendMessage { msg } => {
                        let packat = encode(&OscPacket::Message(msg.into())).unwrap();
                        let packet: String = packat
                            .iter()
                            .map(|b| b.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                            + ";";
                        if let Err(err) = stream.write_all(packet.as_bytes()).await {
                            // WARN: not sure how to deal with failure
                            warn!(
                                "Failed to write message {:?}, don't know how to do about it",
                                err
                            );
                            break None;
                        }
                    }
                }
            }
            Err(err) = stream.writable() => {
                // WARN: not sure how to deal with failure
                warn!("stream unwritable {:?}, don't know how to do about it", err);
                break None;
            }
            read_ready = stream.readable() => {
                if let Err(err) = read_ready {
                    warn!("socket unreadable {:?}, don't know how to do about it", err);
                    break None;
                }
                let mut buf = [0u8; 16];
                match stream.try_read(&mut buf) {
                    Ok(0) => {
                        warn!("socket connection closed by peer");
                        break None;
                    }
                    Ok(_) => {}
                    Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
                    Err(err) => {
                        warn!("read error {:?}, don't know how to do about it", err);
                        break None;
                    }
                }
            }
        }
    }
}
