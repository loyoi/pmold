use thiserror::Error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git 错误: {0}")]
    GitError(String),

    #[error("TOML 解析错误: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("JSON 解析错误: {0}")]
    Json(#[from] serde_json::Error),

    #[error("正则表达式错误: {0}")]
    Regex(#[from] regex::Error),

    #[error("异步任务错误: {0}")]
    AsyncTaskError(String),

    #[error("Git 初始化错误: {0}")]
    GitInitFailed(String),

    #[error("其他错误: {0}")]
    Other(String),
}

impl From<git2::Error> for CustomError {
    fn from(e: git2::Error) -> Self {
        CustomError::GitError(e.to_string())
    }
}

// 在 error.rs 中确保添加了以下实现（假设 CustomError 定义在此）
impl From<tokio::task::JoinError> for CustomError {
    fn from(e: tokio::task::JoinError) -> Self {
        CustomError::AsyncTaskError(e.to_string())
    }
}
