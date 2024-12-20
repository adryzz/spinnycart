use tracing::{Level, level_filters::LevelFilter};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[cfg(debug_assertions)]
const DEFAULT_LOG_LEVEL: Level = Level::DEBUG;
#[cfg(not(debug_assertions))]
const DEFAULT_LOG_LEVEL: Level = Level::INFO;

/// Initialize logging with tracing_subscriber
///
/// Uses an env filter ('SPINNYCART_LOG' variable), and an optional journald log (if 'SPINNYCART_JOURNALD' is set)
///
/// In a debug build, the default log level is DEBUG, and the logs are 'pretty' (multi-line excessively pretty printer).
///
/// In a release build, the default log level is INFO, and the logs are 'full' (but aren't multi-line)
pub fn tracing_init() -> anyhow::Result<()> {
    let env = tracing_subscriber::EnvFilter::builder()
        .with_default_directive(LevelFilter::from_level(DEFAULT_LOG_LEVEL).into())
        .with_env_var("SPINNYCART_LOG")
        .from_env_lossy();

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_level(true);

    #[cfg(debug_assertions)]
    let fmt_layer = fmt_layer.pretty();

    if std::env::var("SPINNYCART_JOURNALD").is_ok() {
        tracing_subscriber::registry()
            .with(env)
            .with(fmt_layer)
            .with(tracing_journald::layer()?)
            .try_init()?;
    } else {
        tracing_subscriber::registry()
            .with(env)
            .with(fmt_layer)
            .try_init()?;
    }

    Ok(())
}
