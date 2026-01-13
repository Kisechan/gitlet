use serde::{Serialize, Deserialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    // 文件名到其对应 blob ID 的映射
    pub files: HashMap<String, String>,
    pub removed: HashSet<String>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            files: HashMap::new(),
            removed: HashSet::new(),
        }
    }
}