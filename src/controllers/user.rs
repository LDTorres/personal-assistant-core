use actix_web::{get, post, web, HttpRequest, Responder};
use log;
use mongodb::Database;
use std::{io::Result, rc};

use crate::{
    respositories::user::MongoUserRepository,
    services::user::UserService,
};

pub struct UserAppData {
    service: rc::Rc<UserService>,
}

#[get("/")]
pub async fn get_users(_: HttpRequest, data: web::Data<UserAppData>) -> Result<impl Responder> {
    log::debug!("get_users");

    match data.service.get_users().await {
        Ok(users) => Ok(web::Json(users)),
        Err(err) => Err(err)
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct CreateUserDTO {
    pub name: String,
    pub phone: String
}

#[post("/")]
pub async fn create_user(user: web::Json<CreateUserDTO>, data: web::Data<UserAppData>) -> Result<impl Responder> {
    log::debug!("create_user");

    let create_user_dto = user.into_inner();

    match data.service.create_user(create_user_dto).await {
        Ok(user) => Ok(web::Json(user)),
        Err(err) => Err(err)
    }
}

#[get("/{id}")]
pub async fn get_user(req: HttpRequest, data: web::Data<UserAppData>) -> Result<impl Responder> {
    log::debug!("get_user");

    let user_id = req.match_info().get("id").unwrap();

    log::debug!("user's id: {}", &user_id);

    match data.service.get_user(user_id).await {
        Ok(user) => Ok(web::Json(user)),
        Err(err) => Err(err)
    }
}

pub fn config(conn: Database) -> web::Data<UserAppData> {
    log::debug!("Register users config");

    let user_repository = rc::Rc::new(MongoUserRepository::new(conn));
    let user_service = rc::Rc::new(UserService::new(user_repository));

    web::Data::new(UserAppData {
        service: user_service,
    })
}

pub fn scope(cfg: &mut web::ServiceConfig, conn: Database) {
    log::info!("Register users controllers");

    cfg
        .app_data(config(conn))
        .service(get_users)
        .service(get_user)
        .service(create_user);
}