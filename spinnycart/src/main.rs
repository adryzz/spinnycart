mod config;
mod log;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log::tracing_init()?;

    let config = config::load_config_file().await?;

    tracing::info!("spinnycart UnifiedPush server started.");

    Ok(())
}
