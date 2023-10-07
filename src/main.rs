// App Layers
mod config;
mod domain;
mod controllers;
mod services;
mod respositories;

// Imports
use actix_web::{App, web, HttpServer};
use envy;
use config::Configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = envy::from_env::<Configuration>()
    .expect("Please provide env vars");

    HttpServer::new(|| {
        App::new()
        .service(controllers::app::home)
        .service(
    web::scope("/api")
                .service(controllers::app::home)
                .service(
                    web::scope("/users").configure(controllers::user::scope)         
                )
        )
    })
    .bind((config.host, config.port))?
    .run()
    .await
}