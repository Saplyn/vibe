use std::time::Duration;

use futures::executor::block_on;
use tokio::{
    pin, select, spawn,
    sync::{mpsc, watch},
    task::JoinHandle,
    time::{Instant, sleep_until},
};
use tracing::trace;

const DEFAULT_BPM: f32 = 120.0;

#[derive(Debug)]
pub struct Ticker {
    bpm: f32,
    cycle: Option<usize>,
    playing: bool,

    action_tx: mpsc::Sender<TickerAction>,
    tick_rx: watch::Receiver<Option<usize>>,
    handler: JoinHandle<()>,
}

impl Default for Ticker {
    fn default() -> Self {
        let (tick_tx, tick_rx) = watch::channel(None);
        let (action_tx, action_rx) = mpsc::channel(16);

        Self {
            bpm: DEFAULT_BPM,
            cycle: None,
            playing: false,

            action_tx,
            tick_rx,
            handler: spawn(ticker_fn(DEFAULT_BPM, action_rx, tick_tx)),
        }
    }
}

impl Ticker {
    pub fn subscribe(&self) -> watch::Receiver<Option<usize>> {
        self.tick_rx.clone()
    }

    pub fn play(&mut self) {
        if !self.playing {
            block_on(self.action_tx.send(TickerAction::Play))
                .expect("Ticker's action channel unexpectedly closed");
            self.playing = true;
        }
    }
    pub fn pause(&mut self) {
        if self.playing {
            block_on(self.action_tx.send(TickerAction::Pause))
                .expect("Ticker' action channel unexpectedly closed");
            self.playing = false;
        }
    }
    pub fn stop(&mut self) {
        block_on(self.action_tx.send(TickerAction::Stop))
            .expect("Ticker' action channel unexpectedly closed");
        self.playing = false;
    }

    pub fn get_bpm(&self) -> f32 {
        self.bpm
    }
    pub fn set_bpm(&mut self, bpm: f32) {
        block_on(self.action_tx.send(TickerAction::SetBPM(bpm)))
            .expect("Ticker' action channel unexpectedly closed");
        self.bpm = bpm;
    }

    pub fn get_cycle(&self) -> Option<usize> {
        self.cycle
    }
    pub fn set_cycle(&mut self, cycle: Option<usize>) {
        block_on(self.action_tx.send(TickerAction::SetCycle(cycle)))
            .expect("Ticker' action channel unexpectedly closed");
        self.cycle = cycle;
    }
}

enum TickerAction {
    Play,
    Pause,
    Stop,

    SetBPM(f32),
    SetCycle(Option<usize>),
}

async fn ticker_fn(
    bpm: f32,
    mut action_rx: mpsc::Receiver<TickerAction>,
    tick_tx: watch::Sender<Option<usize>>,
) {
    let mut cycle: Option<usize> = None;
    let mut count: usize = 0;

    let mut interval = Duration::from_secs_f32(60.0 / (4.0 * bpm));
    let mut next_tick = Instant::now() + interval;
    let mut playing = false;
    let mut remaining = interval;

    loop {
        let sleep_fut = sleep_until(next_tick);
        pin!(sleep_fut);

        select! {
            _ = &mut sleep_fut, if playing => {
                trace!("tick: {}", count);
                tick_tx.send(Some(count))
                    .expect("Tick channel unexpectedly closed");
                if let Some(cycle) = cycle {
                    count = if count + 1 == cycle { 0 } else { count + 1 };
                } else {
                    count += 1;
                }
                next_tick = Instant::now() + interval;
                remaining = interval;
            }

            Some(action) = action_rx.recv() => {
                match action {
                    TickerAction::Play => {
                        if !playing {
                            playing = true;
                            next_tick = Instant::now() + remaining;
                        }
                    }
                    TickerAction::Pause => {
                        if playing {
                            playing = false;
                            remaining = next_tick
                                .saturating_duration_since(Instant::now());
                        }
                    }
                    TickerAction::Stop => {
                        playing = false;
                        remaining = interval;
                        count = 0;
                    },

                    TickerAction::SetBPM(new_bpm) => {
                        interval = Duration::from_secs_f32(60.0 / (4.0 * new_bpm));
                        next_tick = Instant::now() + interval;
                        remaining = interval;
                    },
                    TickerAction::SetCycle(new_cycle) => {
                        cycle = new_cycle;
                    },
                }
            }
        }
    }
}
