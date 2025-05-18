use std::{collections::HashMap, pin::pin, sync::Arc, time::Duration};

use tokio::{
    select,
    sync::{RwLock as AsyncRwLock, mpsc, watch},
    time::{Instant, sleep_until},
};
use tracing::{info, trace, warn};
use vibe_types::models::Pattern;

use crate::controller::ControllerState;

#[derive(Debug)]
pub enum TickerCommand {
    Play,
    Pause,
    Stop,
    SetBPM { bpm: f32 },
}

#[derive(Debug, Clone)]
pub struct TickerState {
    pub patterns: Arc<AsyncRwLock<HashMap<String, Pattern>>>,
    pub bpm: Arc<AsyncRwLock<f32>>,
    pub playing: Arc<AsyncRwLock<bool>>,
}

#[derive(Debug)]
pub struct TickerArg {
    pub cmd_rx: mpsc::Receiver<TickerCommand>,
    pub tick_tx: watch::Sender<(Option<usize>, usize)>,
    pub controller_state: ControllerState,
}

pub async fn main(state: TickerState, arg: TickerArg) {
    info!("Ticker started");

    let TickerState {
        patterns,
        bpm,
        playing,
    } = state;
    let TickerArg {
        mut cmd_rx,
        tick_tx,
        controller_state,
    } = arg;

    let mut interval = Duration::from_secs_f32(60.0 / (4.0 * *bpm.read().await));
    let mut next_tick = Instant::now() + interval;
    let mut remaining = interval;
    let mut tick: Option<usize> = None;

    loop {
        let sleep_fut = sleep_until(next_tick);
        let mut sleep_fut = pin!(sleep_fut);

        select! {
            _ = &mut sleep_fut, if *playing.read().await => {
                if tick.is_none() {
                    tick = Some(0);
                }
                let pattern_name = controller_state.context.read().await;
                if let Some(name) = &*pattern_name {
                    // pattern
                    let cycle = patterns.read().await.get(name).map(|pat| pat.page_count);
                    match cycle {
                        None => {
                            warn!("Pattern not found: {}", name);
                            next_tick = Instant::now() + interval;
                            remaining = interval;
                            continue;
                        }
                        Some(0) => {
                            warn!("Pattern length is 0");
                            next_tick = Instant::now() + interval;
                            remaining = interval;
                            continue;
                        }
                        Some(cycle) => {
                            trace!("tick {:?}!", tick);
                            let limit = 4 * cycle - 1;
                            if tick.unwrap() > limit {
                                tick = Some(limit);
                            }
                            if let Err(err) = tick_tx.send((tick, limit)) {
                                warn!("Ticker failed to send tick: {}", err);
                            };
                            tick = tick.map(|val| if val >= limit { 0 } else { val + 1 });
                        }
                    }
                } else {
                    // track
                    trace!("tick {:?}!", tick);
                    if tick.unwrap() > 15 {
                        tick = Some(15);
                    }
                    if let Err(err) = tick_tx.send((tick, 15)) {
                        warn!("Ticker failed to send tick: {}", err);
                    };
                    tick = tick.map(|val| if val >= 15 { 0 } else { val + 1 });
                };

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
                        tick = None;
                        if let Err(err) = tick_tx.send((None, 0)) {
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
                }
            }
        }
    }
}
