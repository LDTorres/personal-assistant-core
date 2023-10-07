// App Layers
mod config;
mod controllers;
mod domain;
mod respositories;
mod services;

// Imports
use actix_web::{web, App, HttpServer};
use dotenvy;

// Configs
use config::Config;

use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init env
    if let Ok(_) = dotenvy::dotenv() {
        // Init logger
        env_logger::init();

        info!("Envrioment variables loaded!");
    }

    let config = Config::get_config();

    let result = HttpServer::new(|| {
        App::new().service(controllers::app::home).service(
            web::scope("/api")
                .service(controllers::app::home)
                .service(web::scope("/users").configure(controllers::user::scope)),
        )
    })
    .workers(config.api.num_workers)
    .bind((config.api.host.to_string(), config.api.port))?
    .run();

    info!(
        "Server running at: http://{}:{}/",
        config.api.host, config.api.port
    );

    result.await
}
