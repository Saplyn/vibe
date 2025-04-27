use std::{
    net::SocketAddr,
    sync::{Arc, RwLock},
};

use axum::{
    Router, ServiceExt,
    extract::{
        ConnectInfo, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::get,
};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

// LYN: Main

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("vibed=trace"))
        .init();

    let state = Arc::new(RwLock::new(AppState::default()));

    let app = Router::new().route("/", get(ws_handler)).with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

// LYN: State

#[derive(Debug, Default)]
struct AppState {}

// LYN: Handlers

async fn ws_handler(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, addr: SocketAddr) {
    info!("Client connected: {}", addr);

    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                info!("Received text from {}: {}", addr, text);
            }
            Message::Close(_) => {
                info!("Client {} disconnected", addr);
                break;
            }
            _ => {}
        }
    }
}
