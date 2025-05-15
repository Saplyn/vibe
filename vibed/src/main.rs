use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use app::handle_connection;
use axum::{
    Router,
    extract::{ConnectInfo, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use command::ClientCommand;
use communicator::CommunicatorState;
use state::{AppState, Pattern, Track};
use ticker::TickerState;
use tokio::{
    spawn,
    sync::{broadcast, mpsc, watch},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

mod app;
mod command;
mod communicator;
mod mosc;
mod state;
mod ticker;

const VIBED_SERVER_ADDR: &str = "0.0.0.0:8000";
const DEFAULT_BPM: f32 = 120.0;
const DEFAULT_NAME: &str = "Unnamed";
const DEFAULT_TARGET_ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("{}=info,vibed=trace"))
        .init();

    let state = prepare_app();
    let router = Router::new().route("/", get(ws_handler)).with_state(state);
    let listener = tokio::net::TcpListener::bind(VIBED_SERVER_ADDR)
        .await
        .unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    state: State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket, addr, state.0))
}

fn prepare_app() -> AppState {
    // LYN: Spawn Ticker
    let (ticker_cmd_tx, ticker_cmd_rx) = mpsc::channel(32);
    let (tick_tx, tick_rx) = watch::channel(None);
    spawn(ticker::main(TickerState {
        bpm: Arc::new(RwLock::new(DEFAULT_BPM)),
        playing: Arc::new(RwLock::new(false)),
        cmd_rx: ticker_cmd_rx,
        tick_tx,
    }));

    // LYN: Spawn Communicator
    let patterns: Arc<RwLock<HashMap<String, Pattern>>> = Default::default();
    let tracks: Arc<RwLock<HashMap<String, Track>>> = Default::default();
    let (communicator_cmd_tx, communicator_cmd_rx) = mpsc::channel(32);
    spawn(communicator::main(CommunicatorState {
        patterns: patterns.clone(),
        tracks: tracks.clone(),
        target_addr: Arc::new(RwLock::new(DEFAULT_TARGET_ADDR.to_string())),
        context: None,
        cmd_rx: communicator_cmd_rx,
        tick_rx,
    }));

    // LYN: Construct App State
    let (client_cmd_que, _) = broadcast::channel::<ClientCommand>(64);
    AppState {
        name: Arc::new(RwLock::new(DEFAULT_NAME.to_string())),
        patterns,
        tracks,
        ticker_cmd_tx,
        communicator_cmd_tx,
        client_cmd_que,
    }
}
