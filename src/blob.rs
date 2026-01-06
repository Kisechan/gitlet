use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    pub fn new(content: Vec<u8>) -> Self {
        Blob { content }
    }

    pub fn get_data(&self) -> Vec<u8> {
        bincode::serialize(self).expect("无法序列化 blob")
    }

    pub fn get_id(&self) -> String {
        crate::utils::sha1(&[&self.get_data()])
    }
}