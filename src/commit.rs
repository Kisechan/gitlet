// 表示一个 gitlet commit 对象
// TODO: 这个类在更高的层次上做了什么，最好给一个描述
//
// @author TODO

use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::utils::sha1;

// TODO: 在这里添加需要的任何导入

// Gitlet Commit 结构
#[derive(Debug, Serialize, Deserialize)]
pub struct Commit {
    // TODO: 在这里添加实例变量
    //
    // 在这里列出 Commit 类的所有实例变量，上面有有用的
    // 注释说明该变量代表什么以及如何使用该变量。我们已经为您提供了一个关于 `message` 的示例。
    
    // 此 Commit 的消息
    message: String,
    timestamp: u64,
    parent: Option<String>,
    tree: HashMap<String, String>,
}

impl Commit {
    // TODO: 填写这个类的其余部分
    pub fn initial() -> Self {
        Commit {
            message: "initial commit".to_string(),
            timestamp: 0,
            parent: None,
            tree: HashMap::new(),
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        bincode::serialize(self).expect("无法序列化提交")
    }

    pub fn get_id(&self) -> String {
        sha1(&[&self.get_data()])
    }
}
