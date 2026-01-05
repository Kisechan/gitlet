// 表示一个 gitlet 仓库
// TODO: 这个类在更高的层次上做了什么，最好给一个描述
//
// @author TODO

use std::path::{PathBuf};

use crate::commit::Commit;
use crate::utils::*;

// Gitlet 仓库结构
pub struct Repository {
    // TODO: 在这里添加实例变量
    //
    // 在这里列出 Repository 类的所有实例变量，上面有有用的
    // 注释说明该变量代表什么以及如何使用该变量。我们已经为您提供了两个示例。
}

impl Repository {

    pub fn new () -> Self {
        Repository {}
    }

    // 当前工作目录
    pub fn cwd() -> PathBuf {
        std::env::current_dir().expect("无法获取当前工作目录")
    }
    
    // .gitlet 目录
    pub fn gitlet_dir() -> PathBuf {
        Self::cwd().join(".gitlet")
    }

    pub fn objects_dir() -> PathBuf {
        Self::gitlet_dir().join("objects")
    }

    pub fn commits_dir() -> PathBuf {
        Self::objects_dir().join("commits")
    }

    pub fn blobs_dir() -> PathBuf {
        Self::objects_dir().join("blobs")
    }

    pub fn refs_dir() -> PathBuf {
        Self::gitlet_dir().join("refs")
    }

    pub fn refs_heads_dir() -> PathBuf {
        Self::refs_dir().join("heads")
    }

    pub fn refs_tags_dir() -> PathBuf {
        Self::refs_dir().join("tags")
    }

    pub fn head_file() -> PathBuf {
        Self::gitlet_dir().join("HEAD")
    }
    
    pub fn exists(&self) -> bool {
        Self::gitlet_dir().exists() && Self::gitlet_dir().is_dir()
    }

    pub fn set_head(&self, value: &str) {
        write_contents(Self::head_file(), &[value.as_bytes()]).unwrap();
    }

    pub fn create_branch(&self, name: &str, commit_id: &str) {
        let path = join(Self::refs_heads_dir(), &[name]);
        write_contents(path, &[commit_id.as_bytes()]).expect("无法写入分支文件");
    }

    pub fn init_dirs(&self) {
        std::fs::create_dir_all(Self::objects_dir()).expect("无法创建 objects 目录");
        std::fs::create_dir_all(Self::refs_dir()).expect("无法创建 refs 目录");
        std::fs::create_dir_all(Self::refs_heads_dir()).expect("无法创建 refs/heads 目录");
        std::fs::create_dir_all(Self::refs_tags_dir()).expect("无法创建 refs/tags 目录");
    }

    pub fn save_commit(&self, commit: &Commit) {
        let data = commit.get_data();
        let commit_id = commit.get_id();

        write_contents(Self::commits_dir().join(&commit_id), &[&data]).expect("无法写入提交文件");
    }
 
    // TODO: 填写这个类的其余部分
}
