use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use tokio::{
    signal, spawn,
    sync::{RwLock as AsyncRwLock, broadcast, mpsc, watch},
};
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::build::print_built_info;
use crate::command::ClientCommand;
use crate::communicator::{CommunicatorArg, CommunicatorState};
use crate::controller::{ControllerArg, ControllerState};
use crate::handler::{HandlerState, ws_upgrader};
use crate::store::Store;
use crate::ticker::{TickerArg, TickerState};

mod build;
mod command;
mod communicator;
mod controller;
mod handler;
mod models;
mod mosc;
mod store;
mod ticker;

const VIBED_SERVER_ADDR: &str = "0.0.0.0:8000";
const DEFAULT_BPM: f32 = 120.0;
const DEFAULT_NAME: &str = "Unnamed";
const DEFAULT_TARGET_ADDR: &str = "127.0.0.1:8001";
static DEFAULT_SAVE_PATH: &str = "./vibe-store.json";

#[tokio::main]
async fn main() {
    print_built_info();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("vibed=trace"))
        .init();

    let store = Store::load(DEFAULT_SAVE_PATH);
    let state = init_state(store.clone());
    let router = Router::new().route("/", get(ws_upgrader)).with_state(state);
    let listener = tokio::net::TcpListener::bind(VIBED_SERVER_ADDR)
        .await
        .unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(store))
    .await
    .unwrap();
}

fn init_state(store: Store) -> HandlerState {
    // LYN: States
    let controller_state = ControllerState {
        context: Arc::new(AsyncRwLock::new(None)),
    };
    let ticker_state = TickerState {
        patterns: store.patterns.clone(),
        bpm: store.bpm.clone(),
        playing: Arc::new(AsyncRwLock::new(false)),
    };
    let communicator_state = CommunicatorState {
        target_addr: store.target_addr.clone(),
        connected: Arc::new(AsyncRwLock::new(false)),
    };

    // LYN: Channels
    let (ticker_cmd_tx, ticker_cmd_rx) = mpsc::channel(32);
    let (tick_tx, tick_rx) = watch::channel((None, 0));
    let (communicator_cmd_tx, communicator_cmd_rx) = mpsc::channel(32);
    let (connection_status_tx, connection_status_rx) = watch::channel(false);
    let (controller_cmd_tx, controller_cmd_rx) = mpsc::channel(32);
    let (client_cmd_broadcast, _) = broadcast::channel::<ClientCommand>(64);

    // LYN: Spawn Ticker
    spawn(ticker::main(
        ticker_state.clone(),
        TickerArg {
            cmd_rx: ticker_cmd_rx,
            tick_tx,
            controller_state: controller_state.clone(),
        },
    ));

    // LYN: Spawn Communicator
    spawn(communicator::main(
        communicator_state.clone(),
        CommunicatorArg {
            cmd_rx: communicator_cmd_rx,
            connection_status_tx,
        },
    ));

    // LYN: Spawn Controller
    spawn(controller::main(
        controller_state.clone(),
        ControllerArg {
            store: store.clone(),
            cmd_rx: controller_cmd_rx,
            tick_rx: tick_rx.clone(),
            communicator_cmd_tx: communicator_cmd_tx.clone(),
            client_cmd_broadcast_tx: client_cmd_broadcast.clone(),
        },
    ));

    // LYN: Construct App State
    HandlerState {
        store,
        tick_rx,
        connection_status_rx,
        ticker_cmd_tx,
        controller_cmd_tx,
        communicator_cmd_tx,
        client_cmd_broadcast,
        ticker_state,
        controller_state,
        communicator_state,
    }
}

async fn shutdown_signal(store: Store) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => graceful_shutdown(store).await,
        _ = terminate => graceful_shutdown(store).await,
    }
}

async fn graceful_shutdown(store: Store) {
    store.save(DEFAULT_SAVE_PATH).await.unwrap();
}
