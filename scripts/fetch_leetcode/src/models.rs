use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CodeSnippet {
    pub lang: String,
    pub lang_slug: String,
    pub code: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuestionMetadata {
    #[serde(rename = "questionFrontendId")]
    pub qid: String,
    pub title: String,
    pub title_slug: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyChallengeData {
    pub question: QuestionMetadata,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActiveDailyChallenge {
    pub active_daily_coding_challenge_question: DailyChallengeData,
}

#[derive(Deserialize, Debug)]
pub struct DailyResponse {
    pub data: ActiveDailyChallenge,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Submission {
    pub id: String,
    pub status_display: String,
    pub lang: Language,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionList {
    pub submissions: Vec<Submission>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionListData {
    pub question_submission_list: SubmissionList,
}

#[derive(Deserialize, Debug)]
pub struct SubmissionListResponse {
    pub data: SubmissionListData,
}


#[derive(Deserialize, Debug)]
pub struct SubmissionCode {
    pub code: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionDetails {
    pub submission_details: SubmissionCode,
}

#[derive(Deserialize, Debug)]
pub struct SubmissionDetailResponse {
    pub data: SubmissionDetails,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "rust")]
    Rust,
    #[serde(rename = "cpp")]
    Cpp,
    #[serde(rename = "python3")]
    Python,
}

impl Language {
    /// Returns the file extension for the given language
    pub fn extension(&self) -> &'static str {
        match self {
            Language::Rust => "rs",
            Language::Cpp => "cpp",
            Language::Python => "py",
        }
    }

    /// Returns the LeetCode-specific slug
    pub fn slug(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Cpp => "cpp",
            Language::Python => "python3",
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rust" | "rs" => Ok(Language::Rust),
            "cpp" | "c++" => Ok(Language::Cpp),
            "python" | "py" | "python3" => Ok(Language::Python),
            _ => Err(format!("Unsupported language: {}", s)),
        }
    }
}

pub struct SyncResult {
    pub code: String,
    pub path: std::path::PathBuf,
}