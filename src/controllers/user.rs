use actix_web::{web, HttpRequest, Responder};
use mongodb::Database;
use std::{io::Result, rc};

use crate::{services::user::{self, UserService}, respositories::user::MongoUserRepository};

pub struct UserAppData {
    service: rc::Rc<user::UserService>,
}

async fn get_users(_: HttpRequest, app_data: web::Data<UserAppData>) -> Result<impl Responder> {
    match app_data.service.get_users().await {
        Ok(users) => Ok(web::Json(users)),
        Err(err) => Err(err)
    }
}

async fn get_user(req: HttpRequest,  app_data: web::Data<UserAppData>) -> Result<impl Responder> {
    let user_id = req.match_info().get("userId").unwrap();

    match app_data.service.get_user(user_id).await {
        Ok(user) => Ok(web::Json(user)),
        Err(err) => Err(err)
    }
}




pub fn scope(cfg: &mut web::ServiceConfig, conn: Database) {
    let user_repository = rc::Rc::new(MongoUserRepository::new(conn));
    let user_service = rc::Rc::new(UserService::new(user_repository));


    cfg.service(
        web::resource("/users")
            .app_data(UserAppData{service: rc::Rc::clone(&user_service)})
            .route(
                web::get().to( get_users)
            ))
            .route(
                "/{user_id}", 
                web::get().to(get_user)
            );
}
