
use std::io::Result;
use mongodb::{self, bson::{doc, oid::ObjectId}, };
use futures::stream::TryStreamExt;
use async_trait::async_trait;

use crate::domain::user;

#[async_trait]
pub trait UserRepository {
    async fn get_users(&self) -> Result<Vec<user::User>>;
    async fn get_user(&self, user_id: &str) ->  Result<user::User>;
}

pub struct MongoUserRepository<'a> {
    pub conn: &'a mongodb::Database
    pub coll: &'a mongodb::Collection<user::User>
}



impl<'a> MongoUserRepository<'a>  {
    fn new(conn: &mongodb::Database) -> Self {
        let coll: &mongodb::Collection<user::User> = &conn.collection::<user::User>("users");

        Self {
            conn,
            coll
        }
    }
}

#[async_trait]
impl<'a> UserRepository for MongoUserRepository<'a> {
    async fn get_user(&self, user_id: &str) -> Result<user::User> {
        let object_id = ObjectId::parse_str(user_id).unwrap();

        let user = self.coll.find_one(doc!{"_id": object_id}, None).await.unwrap().unwrap();

        Ok(user)
    }

    async fn get_users(&self) -> Result<Vec<user::User>> {
        let mut cursor: mongodb::Cursor<user::User> = self.coll.find(None, None).await.unwrap();

        let mut users: Vec<user::User> = Vec::new();

        while let Some(user) = cursor.try_next().await.unwrap() {
            users.push(user)
        }

        Ok(users)
    }
}