use crate::error::CustomError;
use git2::build::RepoBuilder;
use std::path::Path;

pub async fn clone_repo(repo: &str, path: &Path) -> Result<(), CustomError> {
    let url = if repo.contains("://") || repo.starts_with("git@") {
        repo.to_string()
    } else {
        format!("https://github.com/{}.git", repo)
    };

    let path_buf = path.to_path_buf(); // 获取 path 的所有权副本

    println!("正在克隆仓库: {} 到 {:?}", url, path_buf);

    tokio::task::spawn_blocking(move || -> Result<(), CustomError> {
        RepoBuilder::new()
            .clone(&url, &path_buf) // 使用 path_buf 的引用
            .map_err(|e| CustomError::from(e))?;
        Ok(())
    })
    .await
    .map_err(|e| CustomError::from(e))? // 处理异步任务错误
}
