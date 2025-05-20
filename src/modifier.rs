use crate::error::CustomError;
use crate::guide_parser::{FileModification, Instructions};
use regex::Regex;
use std::path::{Path, PathBuf};

use std::collections::HashMap;

pub fn modify_project(
    base_path: &Path,
    instructions: &Instructions,
    context: &HashMap<String, String>,
) -> Result<(), CustomError> {
    for file_mod in &instructions.files {
        let file_path = base_path.join(&file_mod.path);
        modify_file(&file_path, file_mod, context)?;
    }

    if let Some(app_name) = context.get("app_name") {
        let new_path = base_path;
        if !app_name.is_empty() {
            let parent = new_path.parent().ok_or_else(|| {
                CustomError::Other("Base path has no parent directory".to_string())
            })?;
            let new_path = parent.join(app_name);

            // Rename the directory
            std::fs::rename(base_path, &new_path)
                .map_err(|e| CustomError::Other(format!("Failed to rename directory: {}", e)))?;
        }
    }

    Ok(())
}

fn modify_file(
    path: &PathBuf,
    modification: &FileModification,
    context: &HashMap<String, String>,
) -> Result<(), CustomError> {
    let mut content = std::fs::read_to_string(path)?;

    for replacement in &modification.replacements {
        let replaced = replace_with_context(&replacement.replace, context);
        if replacement.is_regex {
            let re = Regex::new(&replacement.search)?;
            content = re.replace_all(&content, replaced.as_str()).to_string();
        } else {
            content = content.replace(&replacement.search, &replaced);
        }
    }

    std::fs::write(path, content)?;
    Ok(())
}

fn replace_with_context(template: &str, context: &HashMap<String, String>) -> String {
    let mut result = template.to_string();
    for (key, value) in context {
        result = result.replace(&format!("{{{{{}}}}}", key), value);
    }
    result
}

/// 删除原有的 Git 仓库并重新初始化
pub fn reinitialize_git_repo(target_dir: &PathBuf) -> Result<(), CustomError> {
    // 删除 .git 目录
    let git_dir = target_dir.join(".git");
    if git_dir.exists() {
        std::fs::remove_dir_all(&git_dir)?;
        println!("已删除原有 Git 仓库");
    }

    // 初始化新的 Git 仓库
    let status = std::process::Command::new("git")
        .arg("init")
        .arg("--initial-branch")
        .arg("main")
        .current_dir(target_dir)
        .status()?;

    if !status.success() {
        return Err(CustomError::GitInitFailed(
            "git init 命令执行失败".to_string(),
        ));
    }

    println!("已初始化新的 Git 仓库");

    Ok(())
}
