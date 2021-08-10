use color_eyre::Result;
use dotenv::dotenv;
use eyre::WrapErr;
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
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
}
