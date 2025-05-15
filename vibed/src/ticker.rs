use std::{
    pin::pin,
    sync::{Arc, RwLock},
    time::Duration,
};

use tokio::{
    select,
    sync::{mpsc, watch},
    time::{Instant, sleep_until},
};
use tracing::{info, trace};

#[derive(Debug)]
pub enum TickerCommand {
    Play,
    Pause,
    Stop,
    SetBPM { bpm: f32 },
}

#[derive(Debug)]
pub struct TickerState {
    pub bpm: Arc<RwLock<f32>>,
    pub playing: Arc<RwLock<bool>>,
    pub cmd_rx: mpsc::Receiver<TickerCommand>,
    pub tick_tx: watch::Sender<Option<bool>>,
}

pub async fn main(state: TickerState) {
    info!("Ticker started");

    let TickerState {
        bpm,
        playing,
        cmd_rx: mut action_rx,
        tick_tx,
    } = state;

    let mut interval = Duration::from_secs_f32(60.0 / (4.0 * *bpm.read().unwrap()));
    let mut next_tick = Instant::now() + interval;
    let mut remaining = interval;
    let mut tick = 0;

    loop {
        let sleep_fut = sleep_until(next_tick);
        let mut sleep_fut = pin!(sleep_fut);

        select! {
            _ = &mut sleep_fut, if *playing.read().unwrap() => {
                trace!("tick {}!", tick);
                let _ = tick_tx.send(Some(tick == 0));
                tick = if tick == 3 {0} else {tick + 1};
                next_tick = Instant::now() + interval;
                remaining = interval;
            }

            Some(action) = action_rx.recv() => {
                match action {
                    TickerCommand::Play => {
                        let mut playing = playing.write().unwrap();
                        if !*playing {
                            *playing = true;
                            next_tick = Instant::now() + remaining;
                        }
                    }
                    TickerCommand::Pause => {
                        let mut playing = playing.write().unwrap();
                        if *playing {
                            *playing = false;
                            remaining = next_tick
                                .saturating_duration_since(Instant::now());
                        }
                    }
                    TickerCommand::Stop => {
                        *playing.write().unwrap() = false;
                        remaining = interval;
                        tick = 0;
                        let _ = tick_tx.send(None);
                    },
                    TickerCommand::SetBPM { bpm: new_bpm } => {
                        let mut bpm = bpm.write().unwrap();
                        *bpm = new_bpm;
                        interval = Duration::from_secs_f32(60.0 / (4.0 * *bpm));
                        next_tick = Instant::now() + interval;
                        remaining = interval;
                    },
                }
            }
        }
    }
}
