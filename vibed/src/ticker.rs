use std::{pin::pin, sync::Arc, time::Duration};

use tokio::{
    select,
    sync::{RwLock as AsyncRwLock, mpsc, watch},
    time::{Instant, sleep_until},
};
use tracing::{info, trace, warn};

#[derive(Debug)]
pub enum TickerCommand {
    Play,
    Pause,
    Stop,
    SetBPM { bpm: f32 },
    SetCycle { cycle: Option<usize> },
}

#[derive(Debug, Clone)]
pub struct TickerState {
    pub bpm: Arc<AsyncRwLock<f32>>,
    pub playing: Arc<AsyncRwLock<bool>>,
    pub tick: Arc<AsyncRwLock<Option<usize>>>,
    pub cycle: Arc<AsyncRwLock<Option<usize>>>,
}

#[derive(Debug)]
pub struct TickerArg {
    pub cmd_rx: mpsc::Receiver<TickerCommand>,
    pub tick_tx: watch::Sender<Option<usize>>,
}

pub async fn main(state: TickerState, arg: TickerArg) {
    info!("Ticker started");

    let TickerState {
        bpm,
        playing,
        tick,
        cycle,
    } = state;
    let TickerArg {
        mut cmd_rx,
        tick_tx,
    } = arg;

    let mut interval = Duration::from_secs_f32(60.0 / (4.0 * *bpm.read().await));
    let mut next_tick = Instant::now() + interval;
    let mut remaining = interval;

    loop {
        let sleep_fut = sleep_until(next_tick);
        let mut sleep_fut = pin!(sleep_fut);

        select! {
            _ = &mut sleep_fut, if *playing.read().await => {
                let mut tick = tick.write().await;
                if tick.is_none() {
                    *tick = Some(0);
                }
                trace!("tick {:?}!", tick);
                if let Err(err) = tick_tx.send(*tick) {
                    warn!("Ticker failed to send tick: {}", err);
                };
                if let Some(cycle) = *cycle.read().await {
                    *tick = tick.map(|val| if val == 4 * cycle - 1 { 0 } else { val + 1 });
                } else {
                    *tick = tick.map(|val| if val == 15 { 0 } else { val + 1 });
                }
                next_tick = Instant::now() + interval;
                remaining = interval;
            }

            Some(cmd) = cmd_rx.recv() => {
                match cmd {
                    TickerCommand::Play => {
                        let mut playing = playing.write().await;
                        if !*playing {
                            *playing = true;
                            next_tick = Instant::now() + remaining;
                        }
                    }
                    TickerCommand::Pause => {
                        let mut playing = playing.write().await;
                        if *playing {
                            *playing = false;
                            remaining = next_tick
                                .saturating_duration_since(Instant::now());
                        }
                    }
                    TickerCommand::Stop => {
                        *playing.write().await = false;
                        remaining = interval;
                        *tick.write().await = None;
                        if let Err(err) = tick_tx.send(None) {
                            warn!("Ticker failed to send tick: {}", err);
                        };
                    }
                    TickerCommand::SetBPM { bpm: new_bpm } => {
                        let mut bpm = bpm.write().await;
                        *bpm = new_bpm;
                        interval = Duration::from_secs_f32(60.0 / (4.0 * *bpm));
                        next_tick = Instant::now() + interval;
                        remaining = interval;
                    }
                    TickerCommand::SetCycle { cycle: new_cycle } => {
                        *cycle.write().await = new_cycle;
                        *tick.write().await = None;
                        next_tick = Instant::now() + interval;
                        remaining = interval;
                    }
                }
            }
        }
    }
}
