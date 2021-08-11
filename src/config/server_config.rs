use std::time::Duration;
use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;
use sqlx::postgres::PgPool;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub database_url: String
}

impl Config {

    #[instrument]
    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        // This is a subscriber for formatting and logging the tracing data
        // to control what to log we add a filter
        // this will use the RUST_LOG env variable
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading Configuration");

        let mut cfg = config::Config::new();

        // congif can get the config from many sources
        // and merge them
        // here we are setting the source to Environment Variables
        cfg.merge(config::Environment::default())?;

        // to convert the config to a instance of the Config struct
        cfg.try_into()
            .context("loading configuration from environment")
    }

    pub async fn db_pool(&self) -> Result<PgPool> {
        info!("Creating database connection pool");

        PgPool::builder()
        .connect_timeout(Duration::from_secs(30))
        .build(&self.database_url)
        .await
        .context("creating database connection pool")
    }
}
