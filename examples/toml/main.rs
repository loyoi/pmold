use std::{collections::HashMap, path::Path};

use pmold::*;

fn main() -> anyhow::Result<()> {
    let guide_path = Path::new("D:/test/ttclip/guide.toml");
    let instructions = guide_parser::parse_guide(&guide_path)?;

    // 新增：交互式问答
    let mut context = HashMap::new();
    if !instructions.questions.is_empty() {
        println!("=== 开始项目配置 ===");
        context = interactive::ask_questions(&instructions.questions);
    }

    println!("解析结果：{:?}", context);

    Ok(())
}
