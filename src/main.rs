// App Layers
mod config;
mod domain;
mod controllers;
mod services;
mod respositories;

// Imports
use actix_web::{App, web, HttpServer};
use envy;
use dotenvy;
use config::Configuration;
use mongodb::{Database, options::ClientOptions};

async fn connect_database(address: &str, port: &str, user: &str, password: &str, database: &str) -> Database {
    let client_options = ClientOptions::parse(format!("mongodb://{user}:{password}@{address}:{port}/{database}"))
        .await
        .unwrap();

    let client = mongodb::Client::with_options(client_options).unwrap();

    client.database(database)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if let Ok(_) = dotenvy::dotenv() {
        println!("Read envs from file")
    }
    
    let config = envy::from_env::<Configuration>()
        .expect("Please provide env vars");

    let conn = connect_database(
        "0.0.0.0",
        "27017",
        "root",
        "root",
        "core"
    ).await;

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