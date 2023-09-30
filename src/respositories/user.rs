
use std::io::Result;

use crate::domain::user;

pub fn get_users() -> Result<Vec<user::User>> {
    let response = user::User {
        _id: String::from("UP"),
        name: String::from("el tuyo user"),
        phone: 1,
        active: false,
    };

    Ok(vec![response])
}

pub fn get_user(user_id: String) -> Result<user::User> {
    Ok(user::User {
        _id: user_id,
        name: String::from("el tuyo user"),
        phone: 1,
        active: false,
    })
}