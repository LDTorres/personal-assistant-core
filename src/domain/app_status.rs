use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppStatus {
    pub status: String,
    pub commit: String,
}
