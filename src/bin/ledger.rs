use anyhow::Context;
use ledger::{
    config::Config,
    http::routes::{AppState, router},
};
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let config = Config::from_env()?;

    let state = AppState::new(&config).await?;

    sqlx::migrate!()
        .run(&state.pool)
        .await
        .context("failed to apply database migrations")?;
    
    info!("migrations applied");

    let app = router(state);
    let listener = TcpListener::bind(config.bind_addr).await?;
    info!("Server is listening on {}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
