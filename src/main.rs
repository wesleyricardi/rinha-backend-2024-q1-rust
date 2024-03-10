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
    let state = state::get_app_state().await?;

    let port: u16 = std::env::var("PORT")
        .expect("environment variable 'PORT' is not defined")
        .parse()
        .expect("environment variable 'PORT' is invalid");

    HttpServer::new(move || {
        App::new()
            .configure(services_config)
            .app_data(error_handler())
            .app_data(Data::new(state.clone()))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
