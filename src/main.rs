use pmold::{modifier::reinitialize_git_repo, *};

use clap::Parser;
use pmold::error::CustomError;
use std::{collections::HashMap, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // name: String,
    /// GitHub 仓库 URL 或 owner/repo 格式
    repo: String,

    /// 本地目标路径 (默认为当前目录下的 repo 名称)
    #[arg(short, long)]
    target_dir: Option<PathBuf>,

    /// 指引文件路径 (默认为 guide.toml)
    #[arg(short, long, default_value = "guide.toml")]
    guide_file: String,
}

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let args = Args::parse();

    // 确定目标路径
    let target_dir = match args.target_dir {
        Some(dir) => dir,
        None => {
            // 从 repo 中提取项目名作为目录名
            let repo_name = args.repo.split('/').last().unwrap_or("my-project");
            PathBuf::from(repo_name)
        }
    };

    // 获取当前工作目录的绝对路径
    let current_dir = std::env::current_dir()?;
    let target_dir = if target_dir.is_relative() {
        current_dir.join(target_dir)
    } else {
        target_dir
    };

    println!("将在以下位置创建项目: {}", target_dir.display());

    // 1. 克隆项目
    cloner::clone_repo(&args.repo, &target_dir).await?;

    reinitialize_git_repo(&target_dir)?;

    // 2. 读取指引文件
    let guide_path = target_dir.join(&args.guide_file);
    let instructions = guide_parser::parse_guide(&guide_path)?;

    // 新增：交互式问答
    let mut context = HashMap::new();
    if !instructions.questions.is_empty() {
        println!("=== 开始项目配置 ===");
        context = interactive::ask_questions(&instructions.questions);
    }

    // 3. 执行修改（需要传入context）
    modifier::modify_project(&target_dir, &instructions, &context)?;

    println!("项目定制完成！路径: {}", target_dir.display());
    Ok(())
}
