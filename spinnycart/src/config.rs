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
    network: NetworkConf,
    #[config(nested)]
    limits: LimitsConf,
}

#[derive(Config, Debug, Clone)]
pub struct NetworkConf {
    #[config(env = "HTTP_PORT", default = 8080)]
    http_port: u16,

    #[config(env = "ADDRESS", default = "127.0.0.1")]
    address: IpAddr,

    #[config(env = "BEHIND_PROXY", default = "false")]
    behind_proxy: bool,

    #[config(env = "BASE_URL", default = "http://127.0.0.1:8080")]
    base_url: String,
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
