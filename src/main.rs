use actix_web::{web::Data, App, HttpServer};
use config::{error_handler, services::services_config};

mod config;
mod controllers;
mod dtos;
mod error;
mod models;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::log::setup_logger().expect("log configuration failed!");
    let state = state::get_app_state().await?;

    HttpServer::new(move || {
        App::new()
            .configure(services_config)
            .app_data(error_handler())
            .app_data(Data::new(state.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
