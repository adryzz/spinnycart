use std::{net::SocketAddr, sync::Arc};

use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::trace::TraceLayer;

mod config;
mod log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::tracing_init()?;

    let config = Arc::new(config::load_config_file().await?);

    tracing::info!("spinnycart UnifiedPush server started.");

    let state = Arc::new(WebState {
        config: config.clone(),
    });

    let app = Router::new()
        .route("/", get(|| async { "test" }))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    
    let service = app.into_make_service_with_connect_info::<SocketAddr>();

    if config.network.https() {
        let rustls = RustlsConfig::from_pem_file(
            config.network.tls_cert_path.as_deref().unwrap(),
            config.network.tls_cert_path.as_deref().unwrap(),
        ).await?;

        let addr = SocketAddr::new(
            config.network.address,
            config.network.http_port,
        );

        tracing::info!("Listening HTTPS on {}...", addr);

        axum_server::bind_rustls(addr, rustls).serve(service).await?;
    } else {
        let listener = tokio::net::TcpListener::bind(SocketAddr::new(
            config.network.address,
            config.network.http_port,
        ))
        .await?;
        tracing::info!("Listening HTTP on {}...", listener.local_addr()?);
    
        axum::serve(
            listener,
            service,
        )
        .await?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct WebState {
    pub config: Arc<config::Conf>,
}

async fn redirect_http_to_https(config: Arc<config::Conf>) {
    // https://github.com/tokio-rs/axum/blob/main/examples/tls-rustls/src/main.rs
}