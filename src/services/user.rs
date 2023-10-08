
use std::io::Result;

use crate::domain::user::UserRepository;
use crate::domain;

pub struct UserService<'a> {
    repository: &'a dyn UserRepository
}

impl UserService<'_> {
    pub fn new(repository: &impl UserRepository) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_users(&self) -> Result<Vec<domain::user::User>> {
        self.repository.get_users().await
    }
    
    pub async fn get_user(&self, user_id: &str) -> Result<domain::user::User> {
        self.repository.get_user(user_id).await
    }
}