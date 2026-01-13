// 表示一个 gitlet 仓库
// TODO: 这个类在更高的层次上做了什么，最好给一个描述
//
// @author TODO

use std::path::{PathBuf};
use crate::commit::Commit;
use crate::utils::*;
use crate::index::Index;
use crate::blob::Blob;

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

    pub fn index_file() -> PathBuf {
        Self::gitlet_dir().join("index")
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
        std::fs::create_dir_all(Self::gitlet_dir()).expect("无法创建 .gitlet 目录");
        std::fs::create_dir_all(Self::objects_dir()).expect("无法创建 objects 目录");
        std::fs::create_dir_all(Self::commits_dir()).expect("无法创建 commits 目录");
        std::fs::create_dir_all(Self::blobs_dir()).expect("无法创建 blobs 目录");
        std::fs::create_dir_all(Self::refs_dir()).expect("无法创建 refs 目录");
        std::fs::create_dir_all(Self::refs_heads_dir()).expect("无法创建 refs/heads 目录");
        std::fs::create_dir_all(Self::refs_tags_dir()).expect("无法创建 refs/tags 目录");
    }

    pub fn load_commit(&self, commit_id: &str) -> Commit {
        let data = read_contents(Self::commits_dir().join(commit_id)).expect("无法读取提交文件");
        bincode::deserialize(&data).expect("无法反序列化提交对象")
    }

    pub fn save_commit(&self, commit: &Commit) {
        let data = commit.get_data();
        let commit_id = commit.get_id();

        write_contents(Self::commits_dir().join(&commit_id), &[&data]).expect("无法写入提交文件");
    }

    pub fn load_index(&self) -> Index {
        if Self::index_file().exists() {
            let data = read_contents(Self::index_file()).expect("无法读取索引文件");
            serde_json::from_slice(&data).unwrap_or_else(|_| Index::new())
        } else {
            Index::new()
        }
    }

    pub fn save_index(&self, index: &Index) {
        let data = serde_json::to_vec(index).expect("无法序列化索引");
        write_contents(Self::index_file(), &[&data]).expect("无法写入索引文件");
    }

    pub fn load_blob(&self, blob_id: &str) -> Blob {
        let data = read_contents(Self::blobs_dir().join(blob_id)).expect("无法读取 blob 文件");
        bincode::deserialize(&data).expect("无法反序列化提交对象")
    }

    pub fn save_blob(&self, blob: &Blob){
        let data = blob.get_data();
        let blob_id = blob.get_id();

        write_contents(Self::blobs_dir().join(&blob_id), &[&data]).expect("无法写入 blob 文件");
    }

    pub fn get_head_commit(&self) -> Commit {
        let head_ref = read_contents(Self::head_file()).expect("无法读取 HEAD 文件");
        let head_ref_str = String::from_utf8(head_ref).expect("HEAD 文件内容不是有效的 UTF-8 字符串");
        let head_ref_path = Self::gitlet_dir().join(head_ref_str.trim());
        let commit_id = read_contents(head_ref_path).expect("无法读取 HEAD 指向的引用文件");
        let commit_id_str = String::from_utf8(commit_id).expect("引用文件内容不是有效的 UTF-8 字符串");
        self.load_commit(commit_id_str.trim())
    }

    pub fn add_file(&self, filename: &str) {
        // 如果文件不存在，抛出异常
        let data = read_contents(filename).expect(&format!("读取文件 {} 失败", filename));
        let blob_id = sha1(&[&data]);
        let head_commit = self.get_head_commit();
        let cur_tree = head_commit.tree;
        if let Some(existing_blob_id) = cur_tree.get(filename) {
            if existing_blob_id == &blob_id {
                // 文件未更改
                let mut index = self.load_index();
                index.files.remove(filename);
                self.save_index(&index);
                return;
            }
        }
        self.save_blob(&Blob::new(data));

        let mut index = self.load_index();
        index.files.insert(filename.to_string(), blob_id);
        index.removed.remove(filename);
        self.save_index(&index);
    }

    pub fn rm_file(&self, filename: &str) {
        let mut index = self.load_index();
        let head_commit = self.get_head_commit();
        let staged = index.files.contains_key(filename);
        let tracked = head_commit.tree.contains_key(filename);

        if !staged && !tracked {
            eprintln!("文件 {} 未被跟踪", filename);
            return;
        }
        
        if staged {
            index.files.remove(filename);
        }

        if tracked {
            index.removed.insert(filename.to_string());
            // let file_path = Self::cwd().join(filename);
            // if file_path.exists() {
            // restricted_delete(&file_path)
            //     .unwrap_or_else(|_| {
            //         eprintln!("无法删除文件 {}", filename);
            //         false
            //     });
            // }
        }

        self.save_index(&index);
    }

    pub fn commit(&self, message: &str) {
        if message.trim().is_empty() {
            eprintln!("提交信息不能为空");
            return;
        }

        let index = self.load_index();
        if index.files.is_empty() {
            eprintln!("没有要提交的更改");
            return;
        }

        let head_commit = self.get_head_commit();
        let mut new_tree = head_commit.tree.clone();

        for filename in &index.removed {
            new_tree.remove(filename);
        }

        for (filename, blob_id) in &index.files {
            new_tree.insert(filename.clone(), blob_id.clone());
        }

        let new_commit = Commit::new(
            message.to_string(),
            Some(head_commit.get_id()),
            new_tree,
        );

        self.save_commit(&new_commit);
        let head_ref = read_contents(Self::head_file())
            .expect("无法读取 HEAD 文件");
        write_contents(
            Self::gitlet_dir()
                .join(String::from_utf8(head_ref)
                .expect("HEAD 文件内容无效")
                .trim()
            ),
             &[new_commit.get_id().as_bytes()])
            .expect("无法更新 HEAD 指向的引用文件");
        // 清空索引
        self.save_index(&Index::new());
    }
    

}
