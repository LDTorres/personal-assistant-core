use std::io::Result;
use async_trait::async_trait;
use mongodb::bson::{doc, oid::ObjectId};

use serde::{Serialize, Deserialize};
use crate::{common::serialize_object_id::serialize_object_id, services::dto};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename = "_id",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_object_id"
    )]
    pub id: Option<ObjectId>,
    pub name: String,
    pub phone: String,
    pub active: bool,
}

impl User {
    pub fn new(name: &str, phone: &str) -> Self {
        Self{
            id: None,
            name: name.to_string(),
            phone: phone.to_string(),
            active: true,
        }
    }
}

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self, filters: dto::get_users_dto::GetUsersDTO) -> Result<Vec<User>>;
    async fn get_user(&self, user_id: &str) ->  Result<User>;
    async fn create_user(&self, user: User) -> Result<User>;
}
