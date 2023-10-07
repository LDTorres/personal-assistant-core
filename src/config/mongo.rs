use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct MongoConfig {
    pub port: u16,
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
}