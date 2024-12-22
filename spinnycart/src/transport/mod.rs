mod quic;
mod tls;
mod websocket;

use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{ConnectInfo, State, WebSocketUpgrade},
    response::IntoResponse,
};

use crate::WebState;

pub async fn tls_transport_listen(state: Arc<WebState>) {}

pub async fn quic_transport_listen(state: Arc<WebState>) {}

pub async fn websocket_listen(
    ws: WebSocketUpgrade,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<WebState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket::handle_socket(socket, addr, state))
}
