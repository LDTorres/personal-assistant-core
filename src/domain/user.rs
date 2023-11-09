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
    pub phone: i32,
    pub active: bool,
}

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self, filters: dto::get_users_dto::GetUsersDTO) -> Result<Vec<User>>;
    async fn get_user(&self, user_id: &str) ->  Result<User>;
}
