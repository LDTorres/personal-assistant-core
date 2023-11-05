use std::io::Result;
use async_trait::async_trait;
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub phone: i32,
    pub active: bool,
}

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> Result<Vec<User>>;
    async fn get_user(&self, user_id: &str) ->  Result<User>;
}