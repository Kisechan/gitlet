// 各种实用工具
//
// 仔细阅读此文件，因为它提供了多个有用的实用工具函数
// 来为您节省一些时间。
//
// @author P. N. Hilfinger

use sha1::{Sha1, Digest};
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};

// 完整 SHA-1 UID 作为十六进制数字的长度
pub const UID_LENGTH: usize = 40;

/* SHA-1 哈希值 */

// 返回 VALS 连接的 SHA-1 哈希值，VALS 可以是任何
// 字节数组和字符串的混合
pub fn sha1<T: AsRef<[u8]>>(vals: &[T]) -> String {
    let mut hasher = Sha1::new();
    for val in vals {
        hasher.update(val.as_ref());
    }
    format!("{:x}", hasher.finalize())
}

/* 文件删除 */

// 如果存在且不是目录，删除 FILE。如果 FILE 被删除，
// 返回 true，否则返回 false。除非由 FILE 指定的目录
// 也包含一个名为 .gitlet 的目录，否则拒绝删除 FILE
// 并抛出异常
pub fn restricted_delete<P: AsRef<Path>>(file: P) -> Result<bool, io::Error> {
    let file = file.as_ref();
    let parent = file.parent().unwrap_or_else(|| Path::new("."));
    let gitlet_dir = parent.join(".gitlet");
    
    if !gitlet_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "not .gitlet working directory"
        ));
    }
    
    if !file.is_dir() {
        fs::remove_file(file).map(|_| true)
    } else {
        Ok(false)
    }
}

/* 读写文件内容 */

// 将 FILE 的全部内容作为字节数组返回。FILE 必须
// 是一个普通文件。遇到问题时抛出异常
pub fn read_contents<P: AsRef<Path>>(file: P) -> io::Result<Vec<u8>> {
    let file = file.as_ref();
    if !file.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "must be a normal file"
        ));
    }
    fs::read(file)
}

// 将 FILE 的全部内容作为字符串返回。FILE 必须
// 是一个普通文件。遇到问题时抛出异常
pub fn read_contents_as_string<P: AsRef<Path>>(file: P) -> io::Result<String> {
    let contents = read_contents(file)?;
    String::from_utf8(contents)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))
}

// 将 CONTENTS 中的字节连接的结果写入 FILE，
// 需要时创建或覆盖它。CONTENTS 中的每个对象可能是
// 字符串或字节数组。遇到问题时抛出异常
pub fn write_contents<P: AsRef<Path>>(file: P, contents: &[&[u8]]) -> io::Result<()> {
    let file = file.as_ref();
    if file.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::IsADirectory,
            "cannot overwrite directory"
        ));
    }
    let mut f = fs::File::create(file)?;
    for content in contents {
        f.write_all(content)?;
    }
    Ok(())
}

/* 目录 */

// 返回目录 DIR 中所有普通文件名的列表，按字典顺序排序。
// 如果 DIR 不是目录，返回 None
pub fn plain_filenames_in<P: AsRef<Path>>(dir: P) -> io::Result<Option<Vec<String>>> {
    let dir = dir.as_ref();
    if !dir.is_dir() {
        return Ok(None);
    }
    
    let mut files: Vec<String> = fs::read_dir(dir)?
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_file() {
                    e.file_name().into_string().ok()
                } else {
                    None
                }
            })
        })
        .collect();
    
    files.sort();
    Ok(Some(files))
}

/* 其他文件实用工具 */

// 将 FIRST 和 OTHERS 的连接返回到 File 指示符中，
// 类似于 Path::join 方法
pub fn join<P: AsRef<Path>>(first: P, others: &[&str]) -> PathBuf {
    let mut path = first.as_ref().to_path_buf();
    for other in others {
        path.push(other);
    }
    path
}

/* 错误报告 */

// 返回一个 GitletException，其消息由 MSG 组成
pub fn error(msg: &str) -> GitletError {
    GitletError(msg.to_string())
}

// 打印由 MSG 组成的消息，后跟换行符
pub fn message(msg: &str) {
    println!("{}", msg);
}

/* 自定义错误类型 */

// 表示 Gitlet 错误的自定义错误类型
#[derive(Debug, Clone)]
pub struct GitletError(pub String);

impl std::fmt::Display for GitletError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for GitletError {}
