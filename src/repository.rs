// 表示一个 gitlet 仓库
// TODO: 这个类在更高的层次上做了什么，最好给一个描述
//
// @author TODO

use std::path::{Path, PathBuf};

// TODO: 在这里添加需要的任何导入

// Gitlet 仓库结构
pub struct Repository {
    // TODO: 在这里添加实例变量
    //
    // 在这里列出 Repository 类的所有实例变量，上面有有用的
    // 注释说明该变量代表什么以及如何使用该变量。我们已经为您提供了两个示例。
}

impl Repository {
    // 当前工作目录
    pub fn cwd() -> PathBuf {
        std::env::current_dir().expect("无法获取当前工作目录")
    }
    
    // .gitlet 目录
    pub fn gitlet_dir() -> PathBuf {
        Self::cwd().join(".gitlet")
    }
    
    // TODO: 填写这个类的其余部分
}
