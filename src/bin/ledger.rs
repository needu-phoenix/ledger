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
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = Config::from_env()?;

    let state = AppState::new(&config).await?;

    let app = router(state);
    let listener = TcpListener::bind(config.bind_addr).await?;
    info!("Server is listening on {}", config.bind_addr);
    axum::serve(listener, app).await?;
    Ok(())
}
