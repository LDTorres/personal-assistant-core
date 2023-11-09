use serde::{Serialize, Deserialize};
use mongodb::bson::Document;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUsersDTO {
    pub name: Option<String>,
    pub phone: Option<i32>,
    pub active: Option<bool>,
}

impl Into<Option<Document>> for GetUsersDTO {
    fn into(self) -> Option<Document> {
        let mut filter_document = Document::new();

        if let Some(name) = self.name {
            filter_document.insert("name", name);
        }

        if let Some(phone) = self.phone {
            filter_document.insert("phone", phone);
        }

        if let Some(active) = self.active {
            filter_document.insert("active", active);
        }

        if filter_document.is_empty() {
            None
        } else {
            Some(filter_document)
        }
    }
}
