pub mod api;
pub mod mongo;

use envy;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub api: api::ApiConfig,
    pub mongo: mongo::MongoConfig,
}

impl Config {
    pub fn get_config() -> Config {
        let api = envy::prefixed("API_").from_env::<api::ApiConfig>().expect("Please provide env vars");
        let mongo = envy::prefixed("MONGO_").from_env::<mongo::MongoConfig>().expect("Please provide env vars");

        Config {
            api,
            mongo
        }
    }
}