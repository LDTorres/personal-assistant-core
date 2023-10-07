use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ApiConfig {
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_num_workers")]
    pub num_workers: usize,
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    4000
}

fn default_num_workers() -> usize {
    2
}
