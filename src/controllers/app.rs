
use std::io::Result;
use actix_web::{get, web, Responder};

use crate::domain::app_status;

#[get("/")]
pub async fn home()  -> Result<impl Responder> {
    let response = app_status::AppStatus {
        status: String::from("UP"),
        commit: String::from("el tuyo home"),
    };

    Ok(web::Json(response))
}