use crate::guide_parser::{InteractiveQuestion, QuestionType};
use dialoguer::{Input, Select};
use std::collections::HashMap;

pub fn ask_questions(questions: &[InteractiveQuestion]) -> HashMap<String, String> {
    let mut answers = HashMap::new();

    for question in questions {
        let answer = match &question.question_type {
            QuestionType::String => {
                let mut input = Input::new().with_prompt(&question.prompt);

                if let Some(default) = &question.default {
                    input = input.default(default.clone());
                }

                input.interact().unwrap()
            }
            QuestionType::Select(options) => {
                let items = options.options.as_slice(); // 获取可索引的切片
                let selection = Select::new()
                    .with_prompt(&question.prompt)
                    .items(items)
                    .default(0)
                    .interact()
                    .unwrap();

                items[selection].to_string() // 安全索引并克隆字符串
            }
        };

        answers.insert(question.key.clone(), answer);
    }

    answers
}
