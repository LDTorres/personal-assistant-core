use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(default="default_port")]
    pub port: u16,
    #[serde(default="default_host")]
    pub host: String
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 { 
    4000
}