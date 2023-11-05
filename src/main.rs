// App Layers
mod config;
mod controllers;
mod domain;
mod respositories;
mod services;

// Imports
use actix_web::{web, App, HttpServer};
use dotenvy;
use mongodb::{options::ClientOptions, Database};

// Configs
use config::{mongo::MongoConfig, Config};

use log::info;

async fn connect_database(config: &MongoConfig) -> Database {
    let MongoConfig {
        user,
        password,
        port,
        host,
        database,
    } = config;

    let client_options =
        ClientOptions::parse(format!("mongodb://{user}:{password}@{host}:{port}/"))
            .await
            .unwrap();

    let client = mongodb::Client::with_options(client_options).unwrap();

    let conn = client.database(&database[..]);

    conn
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Init env
    if let Ok(_) = dotenvy::dotenv() {
        println!("Read envs from file");
    }

    env_logger::init();

    let config: Config = Config::get_config();

    let conn = connect_database(&config.mongo).await;

    let result = HttpServer::new(move || {
        App::new().service(controllers::app::home).service(
            web::scope("/api").service(controllers::app::home).service(
                web::scope("/users")
                    .configure(|cfg: &mut web::ServiceConfig| controllers::user::scope(cfg, conn.clone())),
            ),
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
