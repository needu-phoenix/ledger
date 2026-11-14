use anyhow::Context;
use std::{env, net::SocketAddr};
pub struct Config {
    pub database_url: String,
    pub bind_addr: SocketAddr,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Config> {
        let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set")?;

        let bind_addr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:8080".to_string())
            .parse()
            .context("BIND_ADDR must be an IP:port, e.g 127.0.0.1:8080")?;

        Ok(Config {
            database_url,
            bind_addr,
        })
    }
}
