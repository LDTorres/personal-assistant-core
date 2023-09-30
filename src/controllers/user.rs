use actix_web::{get, web, HttpRequest, Responder};
use std::io::Result;

use crate::services::user;

#[get("/")]
async fn get_users() -> Result<impl Responder> {
    match user::get_users() {
        Ok(users) => Ok(web::Json(users)),
        Err(err) => Err(err)
    }
}

#[get("/{userId}")]
async fn get_user(req: HttpRequest) -> Result<impl Responder> {
    let user_id = req.match_info().get("userId").unwrap().to_string();

    match user::get_user(user_id) {
        Ok(user) => Ok(web::Json(user)),
        Err(err) => Err(err)
    }
}

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
}
