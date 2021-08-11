#[macro_use]
extern crate validator_derive;

mod config;
mod handlers;
mod models;

use crate::config::Config;
use crate::handlers::app_config;
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server Configuration");

    let pool = config.db_pool().await.expect("Database configuration");

    let crypto_service = config.crypto_service();

    info!(
        "Strating server at http:://{}:{}/",
        config.host, config.port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(pool.clone())
            .data(crypto_service.clone())
            .configure(app_config)
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;
    Ok(())
}
