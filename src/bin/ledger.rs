use anyhow::Context;
use ledger::{
    config::Config,
    db,
    http::{AppState, routes::router},
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

    let pool = db::connect(&config).await?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("failed to apply database migrations")?;

    info!("migrations applied");

    let state = AppState { pool };
    let app = router(state);
    let listener = TcpListener::bind(config.bind_addr).await?;
    info!("Server is listening on {}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
