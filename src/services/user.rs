
use std::io::Result;
use std::rc;

use crate::domain::user::UserRepository;
use crate::domain;

pub struct UserService {
    repository: rc::Rc<dyn UserRepository>
}

impl UserService {
    pub fn new(repository: rc::Rc<dyn UserRepository>) -> Self {
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