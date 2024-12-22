use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    extract::Host,
    handler::HandlerWithoutStateExt,
    http::{StatusCode, Uri},
    response::Redirect,
    routing::get,
};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::trace::TraceLayer;

mod api;
mod config;
mod hashqueue;
mod log;
mod transport;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::tracing_init()?;

    let config = Arc::new(config::load_config_file().await?);

    tracing::info!("spinnycart UnifiedPush server started.");

    let state = Arc::new(WebState {
        config: config.clone(),
    });

    let app = Router::new()
        .route("/", get(|| async { "TODO: add root web page" }))
        .route("/send_204", get(|| async { StatusCode::NO_CONTENT }))
        .merge(api::router(state.clone()))
        .route("/listen", get(transport::websocket_listen))
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone());

    let service = app.into_make_service_with_connect_info::<SocketAddr>();

    if config.network.tls() {
        if config.network.can_use_tls_transport() {
            tracing::info!("Starting TLS transport...");
            tokio::spawn(transport::tls_transport_listen(state.clone())).await?;
        }
        if config.network.can_use_quic_transport() {
            tracing::info!("Starting QUIC transport...");
            tokio::spawn(transport::quic_transport_listen(state.clone())).await?;
        }
    }

    if config.network.https() {
        let rustls = RustlsConfig::from_pem_file(
            config.network.tls_cert_path.as_deref().unwrap(),
            config.network.tls_cert_path.as_deref().unwrap(),
        )
        .await?;

        tokio::spawn(redirect_http_to_https(config.clone()));

        let addr = SocketAddr::new(config.network.address, config.network.http_port);

        tracing::info!("Listening HTTPS on {}...", addr);

        axum_server::bind_rustls(addr, rustls)
            .serve(service)
            .await?;
    } else {
        let listener = tokio::net::TcpListener::bind(SocketAddr::new(
            config.network.address,
            config.network.http_port,
        ))
        .await?;
        tracing::info!("Listening HTTP on {}...", listener.local_addr()?);

        axum::serve(listener, service).await?;
    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct WebState {
    pub config: Arc<config::Conf>,
}

async fn redirect_http_to_https(config: Arc<config::Conf>) {
    fn make_https(host: String, uri: Uri, ports: (u16, u16)) -> anyhow::Result<Uri> {
        let mut parts = uri.into_parts();

        parts.scheme = Some(axum::http::uri::Scheme::HTTPS);

        if parts.path_and_query.is_none() {
            parts.path_and_query = Some("/".parse().unwrap());
        }

        let https_host = host.replace(&ports.0.to_string(), &ports.1.to_string());
        parts.authority = Some(https_host.parse()?);

        Ok(Uri::from_parts(parts)?)
    }

    let ports = (config.network.http_port, config.network.https_port.unwrap());
    let redirect = move |Host(host): Host, uri: Uri| async move {
        match make_https(host, uri, ports) {
            Ok(uri) => Ok(Redirect::permanent(&uri.to_string())),
            Err(error) => {
                tracing::warn!(%error, "Failed to convert URI to HTTPS");
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };

    let addr = SocketAddr::from((config.network.address, config.network.http_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening HTTP on {}...", addr);
    axum::serve(listener, redirect.into_make_service())
        .await
        .unwrap();
}
