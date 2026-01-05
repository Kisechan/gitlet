use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blob {
    pub name: String,
    pub content: Vec<u8>,
}