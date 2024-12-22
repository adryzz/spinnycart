use std::{net::SocketAddr, sync::Arc};

use axum::extract::ws::WebSocket;
use tracing::instrument;

use crate::WebState;

#[instrument(name = "websocket_connection")]
pub async fn handle_socket(mut socket: WebSocket, who: SocketAddr, state: Arc<WebState>) {
    // TODO: listen for notifications and relay them
    // TODO: when a notification receipt comes back, relay it back

    /*let mut recv = state.tx.subscribe();

    loop {
        select! {
            Ok(msg) = recv.recv() => {
                if let Err(_) = socket.send(Message::Text(msg)).await {
                    break;
                }
            }

            msg = socket.next() => {
                if let Some(Ok(_)) = msg {
                    // we dont care (for now)
                } else {
                    // websocket closed
                    break;
                }
            }
        }
    }*/
}
