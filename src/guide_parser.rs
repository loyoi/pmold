use crate::error::CustomError;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileModification {
    pub path: String,
    pub replacements: Vec<Replacement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Replacement {
    pub search: String,
    pub replace: String,
    #[serde(default)]
    pub is_regex: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instructions {
    pub files: Vec<FileModification>,
    #[serde(default)]
    pub questions: Vec<InteractiveQuestion>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InteractiveQuestion {
    pub key: String,
    pub prompt: String,
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub default: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    String,
    Select(SelectOptions),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelectOptions {
    pub options: Vec<String>,
}

pub fn parse_guide(path: &Path) -> Result<Instructions, CustomError> {
    let content = std::fs::read_to_string(path)?;

    if path.extension().and_then(|s| s.to_str()) == Some("json") {
        serde_json::from_str(&content).map_err(|e| e.into())
    } else {
        toml::from_str(&content).map_err(|e| e.into())
    }
}
