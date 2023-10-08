use actix_web::{get, web, HttpRequest, Responder, http, dev::fn_factory, Handler};
use std::io::Result;

use crate::{services::user::{self, UserService}, respositories::user::MongoUserRepository};

pub struct UserController<'a> {
    service: &'a user::UserService<'a>,
}

impl <'a> UserController<'a> {
    fn new(service: &'a user::UserService) -> Self {
        Self { service }
    }

    async fn get_users(&self) -> Result<impl Responder> {
        match self.service.get_users().await {
            Ok(users) => Ok(web::Json(users)),
            Err(err) => Err(err)
        }
    }

    async fn get_user(&self, req: HttpRequest) -> Result<impl Responder> {
        let user_id = req.match_info().get("userId").unwrap();

        match self.service.get_user(user_id).await {
            Ok(user) => Ok(web::Json(user)),
            Err(err) => Err(err)
        }
    }
}



pub fn scope(cfg: &mut web::ServiceConfig) {
    let user_repository = &MongoUserRepository::new(None)
    let user_service = &UserService::new(user_repository);
    let user_controller = UserController::new(user_service);

    cfg.service(
        web::resource("/users")
            .route(
                web::route().method(http::Method::GET).to(|| async { user_controller.get_users().await })
            ))
            .route(
                "/:user_id", 
                web::route().method(http::Method::GET).to(|http_request: HttpRequest| async { user_controller.get_user(http_request).await })
            );
}
