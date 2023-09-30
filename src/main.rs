// App Layers
mod domain;
mod controllers;
mod services;
mod respositories;

// Imports
use actix_web::{App, web, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}