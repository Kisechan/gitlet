// 表示一个 gitlet 仓库
//
// @author Kisechan

use std::path::{PathBuf};
use crate::commit::Commit;
use crate::utils::*;
use crate::index::Index;
use crate::blob::Blob;
use std::collections::{HashSet, VecDeque};

// Gitlet 仓库结构
pub struct Repository {

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
        // 如果文件不存在，打印错误并返回
        let data = match read_contents(filename) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("File does not exist.");
                return;
            }
        };
        let blob = Blob::new(data);
        let blob_id = blob.get_id();   
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
        self.save_blob(&blob);

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
            eprintln!("No reason to remove the file.");
            return;
        }
        
        if staged {
            index.files.remove(filename);
        }

        if tracked {
            index.removed.insert(filename.to_string());
            let file_path = Self::cwd().join(filename);
            if file_path.exists() {
                let _ = restricted_delete(&file_path);
            }
        }

        self.save_index(&index);
    }

    pub fn commit(&self, message: &str) {
        if message.trim().is_empty() {
            eprintln!("Please enter a commit message.");
            return;
        }

        let index = self.load_index();
        if index.files.is_empty() && index.removed.is_empty() {
            eprintln!("No changes added to the commit.");
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
    
    pub fn log(&self) {
        let mut cur_commit = self.get_head_commit();
        loop {
            println!("{}", cur_commit.get_log());
            match &cur_commit.parent {
                None => break,
                Some(parent_id) => {
                    cur_commit = self.load_commit(parent_id);
                }
            }                    
        }
    }
    
    pub fn global_log(&self) {
        let commit_dir = Self::commits_dir();
        let entries = std::fs::read_dir(commit_dir).expect("无法读取 commits");
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file(){
                    if let Some(commit_id) = path.file_name() {
                        if let Some(commit_id_str) = commit_id.to_str() {
                            // 加载并打印 commit
                            let commit = self.load_commit(commit_id_str);
                            println!("{}", commit.get_log());
                        }
                    }
                }
            }
        }
    }

    pub fn find(&self, message: &str) {
        let commit_dir = Self::commits_dir();
        let entries = std::fs::read_dir(commit_dir).expect("无法读取 commits 目录");
        let mut found = false;
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(commit_id) = path.file_name() {
                        if let Some(commit_id_str) = commit_id.to_str() {
                            let commit = self.load_commit(commit_id_str);
                            if commit.message == message {
                                println!("{}", commit.get_id());
                                found = true;
                            }
                        }
                    }
                }
            }
        }
        if !found {
            println!("Found no commit with that message.");
        }
    }

    fn get_branches(&self) {
        println!("Branches:");
        let head_ref = read_contents(Self::head_file())
            .expect("无法读取 HEAD 文件");
        let head_ref_str = String::from_utf8(head_ref)
            .expect("HEAD 文件内容无效");
        let current_branch = head_ref_str
            .trim()
            .strip_prefix("refs/heads/")
            .unwrap_or("");
        let heads_dir = Self::refs_heads_dir();
        if let Ok(entries) = std::fs::read_dir(heads_dir) {
            let mut branches: Vec<String> = Vec::new();
            for entry in entries.flatten() {
                if let Some(branch_name) = entry.file_name().to_str() {
                    branches.push(branch_name.to_string());
                }
            }
            branches.sort();
            for branch in branches {
                if branch == current_branch {
                    println!("*{}", branch);
                } else {
                    println!("{}", branch);
                }
            }
        }
    }

    fn get_staged_files(&self) {
        println!("Staged Files:");
        let index = self.load_index();
        let mut staged: Vec<&String> = index.files.keys().collect();
        staged.sort();
        for filename in staged {
            println!("{}", filename);
        }
    }

    fn get_removed_files(&self) {
        println!("Removed Files:");
        let index = self.load_index();
        let mut removed: Vec<&String> = index.removed.iter().collect();
        removed.sort();
        for filename in removed {
            println!("{}", filename);
        }
    }

    pub fn status(&self) {
        self.get_branches();
        self.get_staged_files();
        self.get_removed_files();
    }

    fn restore_files_from_commit(&self, commit: &Commit, filename: &str) -> Result<(), String> {
        match commit.tree.get(filename) {
            Some(blob_id) => {
                let blob = self.load_blob(blob_id);
                write_contents(Self::cwd().join(filename), &[&blob.content])
                    .map_err(|e| format!("无法写入文件 {}，错误原因：{}", filename, e))?;
                Ok(())
            }
            None => Err("File does not exist in that commit.".to_string())
        }
    }

    fn find_commit_by_prefix(&self, prefix: &str) -> Result<String, String> {
        let commits_dir = Self::commits_dir();
        let entries = std::fs::read_dir(commits_dir)
            .map_err(|_| "无法读取 commits 目录".to_string())?;
        let mut matches = Vec::new();
        for entry in entries.flatten() {
            if let Some(commit_id) = entry.file_name().to_str() {
                if commit_id.starts_with(prefix) {
                    matches.push(commit_id.to_string());
                }
            }
        }
        match matches.len() {
            0 => Err("No commit with that id exists.".to_string()),
            _ => Ok(matches[0].clone()),
        }
    }

    // 检查是否有 untracked 文件会被覆盖
    fn check_untracked_files(&self, current_commit: &Commit, target_commit: &Commit) -> Result<(), String> {
        let index = self.load_index();
        if let Ok(Some(working_files)) = plain_filenames_in(Self::cwd()) {
            for filename in working_files {
                // 跳过 .gitlet 目录中的文件
                if filename.starts_with(".gitlet") {
                    continue;
                }
                let tracked = current_commit.tree.contains_key(&filename);
                let staged = index.files.contains_key(&filename);
                if !tracked && !staged {
                    if target_commit.tree.contains_key(&filename) {
                        return Err("There is an untracked file in the way; delete it, or add and commit it first.".to_string());
                    }
                }
            }
        }
        Ok(())
    }

    pub fn checkout_file(&self, filename: &str) {
        let head_commit = self.get_head_commit();
        if let Err(msg) = self.restore_files_from_commit(&head_commit, filename) {
            eprintln!("{}", msg);
        }
    }

    pub fn checkout_file_from_commit(&self, commit_id: &str, filename: &str) {
        let full_commit_id = match self.find_commit_by_prefix(commit_id) {
            Ok(id) => id,
            Err(msg) => {
                eprintln!("{}", msg);
                return;
            }
        };
        let commit = self.load_commit(&full_commit_id);
        if let Err(msg) = self.restore_files_from_commit(&commit, filename) {
            eprintln!("{}", msg);
        }
    }

    pub fn checkout_branch(&self, branch_name: &str) {
        let branch_path = Self::refs_heads_dir().join(branch_name);
        if !branch_path.exists() {
            eprintln!("No such branch exists.");
            return;
        }
        let head_ref = read_contents(Self::head_file())
            .expect("无法读取 HEAD 文件");
        let head_ref_str = String::from_utf8(head_ref)
            .expect("HEAD 文件内容无效");
        let current_branch = head_ref_str
            .trim()
            .strip_prefix("refs/heads/")
            .unwrap_or("");
        if current_branch == branch_name {
            eprintln!("No need to checkout the current branch.");
            return;
        }
        let target_commit_id = read_contents_as_string(&branch_path)
            .expect("无法读取分支文件");
        let target_commit = self.load_commit(target_commit_id.trim());
        let head_commit = self.get_head_commit();
        if let Err(msg) = self.check_untracked_files(&head_commit, &target_commit) {
            eprintln!("{}", msg);
            return;
        }
        for filename in head_commit.tree.keys() {
            if !target_commit.tree.contains_key(filename) {
                let file_path = Self::cwd().join(filename);
                if file_path.exists() {
                    let _ = restricted_delete(&file_path);
                }
            }
        }
        for (filename, _) in &target_commit.tree {
            let _ = self.restore_files_from_commit(&target_commit, filename);
        }
        self.set_head(&format!("refs/heads/{}", branch_name));
        self.save_index(&Index::new());
    }

    pub fn branch(&self, branch_name: &str) {
        let branch_path = Self::refs_heads_dir().join(branch_name);
        if branch_path.exists() {
            eprintln!("A branch with that name already exists.");
            return;
        }
        let head_commit = self.get_head_commit();
        let commit_id = head_commit.get_id();
        self.create_branch(branch_name, &commit_id);
    }

    pub fn rm_branch(&self, branch_name: &str) {
        let branch_path = Self::refs_heads_dir().join(branch_name);
        if !branch_path.exists() {
            eprintln!("A branch with that name does not exist.");
            return;
        }
        let head_ref = read_contents(Self::head_file())
            .expect("无法读取 HEAD 文件");
        let head_ref_str = String::from_utf8(head_ref)
            .expect("HEAD 文件内容无效");
        let current_branch = head_ref_str
            .trim()
            .strip_prefix("refs/heads/")
            .unwrap_or("");
        if current_branch == branch_name {
            eprintln!("Cannot remove the current branch.");
            return;
        }
        std::fs::remove_file(branch_path)
            .expect("无法删除分支文件");
    }

    pub fn reset(&self, commit_id: &str) {
        let full_commit_id = self.find_commit_by_prefix(commit_id).unwrap();
        let target_commit = self.load_commit(&full_commit_id);
        let head_commit = self.get_head_commit();
        if let Err(msg) = self.check_untracked_files(&head_commit, &target_commit) {
            eprintln!("{}", msg);
            return;
        }
        for filename in head_commit.tree.keys() {
            if !target_commit.tree.contains_key(filename) {
                let file_path = Self::cwd().join(filename);
                if file_path.exists() {
                    let _ = restricted_delete(&file_path);
                }
            }
        }
        for (filename, _) in &target_commit.tree {
            let _ = self.restore_files_from_commit(&target_commit, filename);
        }
        let head_ref = read_contents(Self::head_file())
            .expect("无法读取 HEAD 文件");
        let head_ref_str = String::from_utf8(head_ref)
            .expect("HEAD 文件内容无效");
        write_contents(
            Self::gitlet_dir().join(head_ref_str.trim()),
            &[full_commit_id.as_bytes()]
        ).expect("无法更新分支指针");
        self.save_index(&Index::new());
    }

    fn find_split_point(&self, commit1_id: &str, commit2_id: &str) -> String {
        // BFS
        let mut ancestors1 = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(commit1_id.to_string());
        while let Some(commit_id) = queue.pop_front() {
            if ancestors1.contains(&commit_id) {
                continue;
            }
            ancestors1.insert(commit_id.clone());
            let commit = self.load_commit(&commit_id);
            if let Some(parent_id) = commit.parent {
                queue.push_back(parent_id);
            }
        }
        let mut current_id = commit2_id.to_string();
        loop {
            if ancestors1.contains(&current_id) {
                return current_id;
            }
            let commit = self.load_commit(&current_id);
            match commit.parent {
                Some(parent_id) => current_id = parent_id,
                None => return current_id,
                // 到达 initial commit
            }
        }
    }
    
    fn merge_commit(&self, message: &str) {
        let index = self.load_index();
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
        self.save_index(&Index::new());
    }

    pub fn merge(&self, branch_name: &str) {
        let index = self.load_index();
        if !index.files.is_empty() || !index.removed.is_empty() {
            eprintln!("You have uncommitted changes.");
            return;
        }
        let branch_path = Self::refs_heads_dir().join(branch_name);
        if !branch_path.exists() {
            eprintln!("A branch with that name does not exist.");
            return;
        }
        let head_ref = read_contents(Self::head_file())
            .expect("无法读取 HEAD 文件");
        let head_ref_str = String::from_utf8(head_ref)
            .expect("HEAD 文件内容无效");
        let current_branch = head_ref_str
            .trim()
            .strip_prefix("refs/heads/")
            .unwrap_or("");
        if current_branch == branch_name {
            eprintln!("Cannot merge a branch with itself.");
            return;
        }
        let current_commit = self.get_head_commit();
        let given_branch_commit_id = read_contents_as_string(&branch_path)
            .expect("无法读取分支文件");
        let given_commit = self.load_commit(given_branch_commit_id.trim());
        let split_point_id = self.find_split_point(
            &current_commit.get_id(),
            &given_commit.get_id()
        );
        let split_commit = self.load_commit(&split_point_id);
        if split_point_id == given_commit.get_id() {
            println!("Given branch is an ancestor of the current branch.");
            return;
        }
        if split_point_id == current_commit.get_id() {
            self.checkout_branch(branch_name);
            println!("Current branch fast-forwarded.");
            return;
        }
        if let Err(msg) = self.check_untracked_files(&current_commit, &given_commit) {
            eprintln!("{}", msg);
            return;
        }
        let mut conflict = false;
        let mut new_index = Index::new();
        let mut all_files = HashSet::new();
        for filename in split_commit.tree.keys() {
            all_files.insert(filename.clone());
        }
        for filename in current_commit.tree.keys() {
            all_files.insert(filename.clone());
        }
        for filename in given_commit.tree.keys() {
            all_files.insert(filename.clone());
        }
        for filename in all_files {
            let split_blob = split_commit.tree.get(&filename);
            let current_blob = current_commit.tree.get(&filename);
            let given_blob = given_commit.tree.get(&filename);
            match (split_blob, current_blob, given_blob) {
                // 在 given 中修改，在 current 中未修改
                (Some(s), Some(c), Some(g)) if s == c && s != g => {
                    self.restore_files_from_commit(&given_commit, &filename).ok();
                    new_index.files.insert(filename.clone(), g.clone());
                }
                 // 在 current 中修改，在 given 中未修改
                (Some(s), Some(c), Some(g)) if s == g && s != c => { }
                (_, Some(c), Some(g)) if c == g => { }
                (Some(_), None, None) => { }
                // 不在 split point，只在 current
                (None, Some(_), None) => { }
                // 不在 split point，只在 given
                (None, None, Some(g)) => {
                    self.restore_files_from_commit(&given_commit, &filename).ok();
                    new_index.files.insert(filename.clone(), g.clone());
                }
                // 在 split point，current 未修改，given 中删除
                (Some(s), Some(c), None) if s == c => {
                    let file_path = Self::cwd().join(&filename);
                    if file_path.exists() {
                        restricted_delete(&file_path).ok();
                    }
                    new_index.removed.insert(filename.clone());
                }
                // 在 split point，given 未修改，current 中删除
                (Some(s), None, Some(g)) if s == g => { }
                // 冲突情况
                _ => {
                    conflict = true;
                    let current_content = if let Some(blob_id) = current_blob {
                        self.load_blob(blob_id).content
                    } else {
                        Vec::new()
                    };
                    let given_content = if let Some(blob_id) = given_blob {
                        self.load_blob(blob_id).content
                    } else {
                        Vec::new()
                    };
                    let mut conflict_content = Vec::new();
                    conflict_content.extend_from_slice(b"<<<<<<< HEAD\n");
                    conflict_content.extend_from_slice(&current_content);
                    conflict_content.extend_from_slice(b"=======\n");
                    conflict_content.extend_from_slice(&given_content);
                    conflict_content.extend_from_slice(b">>>>>>>\n");
                    write_contents(Self::cwd().join(&filename), &[&conflict_content]).ok();
                    let blob = Blob::new(conflict_content);
                    self.save_blob(&blob);
                    new_index.files.insert(filename.clone(), blob.get_id());
                }
            }
        }
        self.save_index(&new_index);
        let commit_message = format!("Merged {} into {}.", branch_name, current_branch);
        self.merge_commit(&commit_message);
        if conflict {
            println!("Encountered a merge conflict.");
        }
    }
}
