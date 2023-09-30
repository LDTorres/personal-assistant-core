
use std::io::Result;

use crate::domain::{self};
use crate::respositories::{self};

pub fn get_users() -> Result<Vec<domain::user::User>> {
    respositories::user::get_users()
}

pub fn get_user(user_id: String) -> Result<domain::user::User> {
    respositories::user::get_user(user_id)
}