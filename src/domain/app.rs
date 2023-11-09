use std::io::Result;
use std::collections::HashMap;

#[derive(Debug)]
pub struct App {
    pub name: String,
    pub metadata: HashMap<String,String>
}

pub trait AppRepository {
    async fn get_apps(&self) -> Result<Vec<App>>;
    async fn get_app(&self, app_id: &str) -> Result<App>;
}