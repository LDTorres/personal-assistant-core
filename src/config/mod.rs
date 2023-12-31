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
        let api = envy::prefixed("API_")
            .from_env::<api::ApiConfig>()
            .expect("Please provide api env vars");
        let mongo = envy::prefixed("MONGO_DB_")
            .from_env::<mongo::MongoConfig>()
            .expect("Please provide mongo env vars");

        Config { api, mongo }
    }
}
