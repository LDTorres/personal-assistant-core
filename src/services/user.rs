
use std::io::Result;
use std::rc;

use crate::domain::user::{UserRepository, User};
use crate::{domain, controllers};

use super::dto;

pub struct UserService {
    repository: rc::Rc<dyn UserRepository>
}

impl UserService {
    pub fn new(repository: rc::Rc<dyn UserRepository>) -> Self {
        Self {
            repository
        }
    }

    pub async fn get_users(&self, filters: dto::get_users_dto::GetUsersDTO) -> Result<Vec<domain::user::User>> {
        self.repository.get_users(filters).await
    }
    
    pub async fn get_user(&self, user_id: &str) -> Result<domain::user::User> {
        self.repository.get_user(user_id).await
    }

    pub async fn create_user(&self, create_user_dto: controllers::user::CreateUserDTO) -> Result<domain::user::User> {
        let controllers::user::CreateUserDTO {name, phone} = create_user_dto;

        let user: User = User::new(&name, &phone);

        self.repository.create_user(user).await
    }
}