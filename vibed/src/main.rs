use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use app::{ServerAction, handle_connection, state::AppState};
use axum::{
    Router,
    extract::{ConnectInfo, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

mod app;
mod osc;
mod ticker;

const VIBED_SERVER_ADDR: &str = "0.0.0.0:8000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("{}=info,vibed=trace"))
        .init();

    let state = Arc::new(RwLock::new(AppState::default()));
    let router = Router::new().route("/", get(ws_handler)).with_state(state);
    let listener = tokio::net::TcpListener::bind(VIBED_SERVER_ADDR)
        .await
        .unwrap();

    error!(
        "{}",
        serde_json::to_string(&ServerAction::PatternAdd {
            name: String::from("uwu")
        })
        .unwrap()
    );

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
    state: State<Arc<RwLock<AppState>>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_connection(socket, addr, state.0))
}
