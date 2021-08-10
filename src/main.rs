mod config;
mod handlers;

use crate::handlers::app_config;
use crate::config::Config;
use actix_web::{middleware::Logger, App, HttpServer};
use color_eyre::Result;
use tracing::info;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = Config::from_env().expect("Server Configuration");

    info!("Strating server at http:://{}:{}/", config.host, config.port);

    HttpServer::new(move || {
            App::new().wrap(Logger::default()).configure(app_config)
        })
        .bind(format!("{}:{}", config.host, config.port))?
        .run()
        .await?;
    Ok(())
}
