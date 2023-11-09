use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{
    self,
    bson::{doc, oid::ObjectId},
};
use std::io::Result;

use crate::{domain::user, services::dto};

pub struct MongoUserRepository {
    pub conn: mongodb::Database,
    pub coll: mongodb::Collection<user::User>,
}

impl MongoUserRepository {
    pub fn new(conn: mongodb::Database) -> Self {
        let coll: mongodb::Collection<user::User> = conn.collection::<user::User>("users");

        Self { conn, coll }
    }
}

#[async_trait]
impl user::UserRepository for MongoUserRepository {
    async fn get_user(&self, user_id: &str) -> Result<user::User> {
        let object_id = ObjectId::parse_str(user_id).unwrap();

        let user = self
            .coll
            .find_one(doc! {"_id": object_id}, None)
            .await
            .unwrap()
            .unwrap();

        Ok(user)
    }

    async fn get_users(&self, filters: dto::get_users_dto::GetUsersDTO) -> Result<Vec<user::User>> {
        let mut cursor: mongodb::Cursor<user::User> = self.coll.find(filters, None).await.unwrap();

        let mut users: Vec<user::User> = Vec::new();

        while let Some(user) = cursor.try_next().await.unwrap() {
            users.push(user);
        }

        Ok(users)
    }

    async fn create_user(&self, mut user: user::User) -> Result<user::User> {
        let result: mongodb::results::InsertOneResult = self.coll.insert_one(&user, None).await.unwrap();

        let inserted_id = result.inserted_id.as_object_id();

        user.id = inserted_id;

        return Ok(user);
    }
}
