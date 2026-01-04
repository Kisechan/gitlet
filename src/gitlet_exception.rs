// 指示 Gitlet 错误的一般异常。对于致命错误，
// .message() 的结果是要打印的错误消息。
// @author P. N. Hilfinger

use std::fmt;

// Gitlet 异常错误
#[derive(Debug, Clone)]
pub struct GitletException {
    // 错误消息
    message: Option<String>,
}

impl GitletException {
    // 没有消息的 GitletException
    pub fn new() -> Self {
        GitletException { message: None }
    }
    
    // 具有指定消息的 GitletException
    pub fn with_message(msg: String) -> Self {
        GitletException { message: Some(msg) }
    }
    
    // 获取错误消息
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }
}

impl Default for GitletException {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for GitletException {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{}", msg),
            None => write!(f, "Gitlet exception"),
        }
    }
}

impl std::error::Error for GitletException {}
