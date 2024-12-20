use confique::Config;
use std::{net::IpAddr, path::PathBuf};

/// Loads config file
///
/// File priority:
///
/// ./spinnycart.toml (DEBUG build only)
///
/// /etc/spinnycart.toml
///
/// todo
pub async fn load_config_file() -> anyhow::Result<Conf> {
    Conf::builder()
        .env()
        .file("spinnycart.toml")
        .file("/etc/spinnycart.toml")
        .load()
        .map_err(|e| e.into())

    // TODO: warn user if no config file was found
    //tracing::info!("No config file found. Using default settings...");
}

#[derive(Config, Debug, Clone)]
pub struct Conf {
    #[config(nested)]
    pub network: NetworkConf,
    #[config(nested)]
    pub limits: LimitsConf,
    #[config(nested)]
    pub auth: AuthConf,
}

#[derive(Config, Debug, Clone)]
pub struct NetworkConf {
    #[config(env = "HTTP_PORT", default = 8080)]
    pub http_port: u16,

    #[config(env = "HTTPS_PORT")]
    pub https_port: Option<u16>,

    #[config(env = "ADDRESS", default = "127.0.0.1")]
    pub address: IpAddr,

    #[config(env = "BEHIND_PROXY", default = false)]
    pub behind_proxy: bool,

    #[config(env = "BASE_URL", default = "http://127.0.0.1:8080")]
    pub base_url: String,

    #[config(env = "TLS_CERT_PATH")]
    pub tls_cert_path: Option<PathBuf>,

    #[config(env = "TLS_KEY_PATH")]
    pub tls_key_path: Option<PathBuf>,
}

#[derive(Config, Debug, Clone)]
pub struct LimitsConf {
    /// Max message size (bytes)
    ///
    /// Default: 4K
    #[config(default = 4096)]
    message_size_limit: u32,

    /// Minimum delay for Scheduled Delivery (seconds)
    ///
    /// Default: 10s
    #[config(default = 10)]
    message_delay_min_limit: u32,

    /// Maximum delay for Scheduled Delivery (seconds)
    ///
    /// Default: 3 days
    #[config(default = 259200)]
    message_delay_max_limit: u32,

    /// Maximum number of topics before the server starts rejecting them
    ///
    /// Default: 15000
    #[config(default = 15000)]
    global_topic_limit: u32,

    /// Maximum number of subscriptions (open connections) per visitor
    ///
    /// Default: 30
    #[config(default = 30)]
    visitor_subscription_limit: u32,
    // TODO: add request limits (when not running behind a proxy)
}

#[derive(Config, Debug, Clone)]
pub struct AuthConf {}


impl NetworkConf {
    pub fn https(&self) -> bool {
        match (self.https_port, &self.tls_cert_path, &self.tls_key_path) {
            (None, None, None) => return false,
            (Some(_), Some(_), Some(_)) => return true,
            _ => {
                if self.https_port.is_none() {
                    tracing::error!("HTTPS is enabled, but no HTTPS port was configured.");
                }
                if self.tls_cert_path.is_none() {
                    tracing::error!("HTTPS is enabled, but no TLS certificate was configured.");
                }

                if self.tls_key_path.is_none() {
                    tracing::error!("HTTPS is enabled, but no TLS certificate key was configured.");
                }

                tracing::warn!("Running in HTTP-only mode.");
                return false
            }
        }
    }
}