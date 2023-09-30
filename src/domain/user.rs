use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub _id: String,
    pub name: String,
    pub phone: i32,
    pub active: bool,
}