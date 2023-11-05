use actix_web::{get, web, HttpRequest, Responder};
use log::info;
use mongodb::Database;
use std::cell::RefCell;
use std::{io::Result, rc};

use crate::{
    respositories::user::MongoUserRepository,
    services::user::{self, UserService},
};

pub struct UserAppData {
    service: RefCell<user::UserService>,
}

#[get("/")]
pub async fn get_users(_: HttpRequest, data: web::Data<UserAppData>) -> Result<impl Responder> {
    info!("get_users");

    match data.service.borrow().get_users().await {
        Ok(users) => Ok(web::Json(users)),
        Err(err) => Err(err)
    }
}

#[get("/{id}")]
pub async fn get_user(req: HttpRequest, data: web::Data<UserAppData>) -> Result<impl Responder> {
    let user_id = req.match_info().get("id").unwrap();

    info!("get_user");

    match data.service.borrow().get_user(user_id).await {
        Ok(user) => Ok(web::Json(user)),
        Err(err) => Err(err)
    }
}

pub fn config(conn: Database) -> web::Data<UserAppData> {
    info!("Register users config");

    let user_repository = rc::Rc::new(MongoUserRepository::new(conn));
    let user_service = RefCell::new(UserService::new(user_repository));

    web::Data::new(UserAppData {
        service: user_service,
    })
}

pub fn scope(cfg: &mut web::ServiceConfig, conn: Database) {
    info!("Register users controllers");

    cfg
    .app_data(config(conn.clone()))
    .service(get_users)
    .service(get_user);
}
