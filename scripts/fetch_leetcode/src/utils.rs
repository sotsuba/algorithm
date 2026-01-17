use std::fs;
use std::error::Error;
use std::path::{Path, PathBuf};
use std::env;
use crate::models::{QuestionMetadata, Language};

fn get_project_dir() -> Result<PathBuf, Box<dyn Error>> {
    let path = env::current_dir()?;

    let project_dir = path.parent()
        .and_then(|p| p.parent())
        .ok_or("Failed to find project root")?;
    
    Ok(project_dir.to_path_buf())
}

pub fn get_problem_dir(question: &QuestionMetadata) -> Result<PathBuf, Box<dyn Error>> {
    let qid = &question.qid;
    let slug = &question.title_slug;

    let formatted_id = match qid.parse::<i32>() {
        Ok(num) => format!("{:05}", num),
        Err(_) => qid.to_lowercase().replace(' ', "_"),
    };
    
    let dir_name = format!("leetcode/{}-{}", formatted_id, slug);
    let mut path = get_project_dir()?;
    path.push(dir_name);
    fs::create_dir_all(&path).expect("Cannot create a directory.");
    Ok(path)
} 

pub fn create_readme(dir_path: &Path, question: &QuestionMetadata) -> Result<(), Box<dyn Error>> {
    let qid = &question.qid;
    let title = &question.title;
    let slug = &question.title_slug;
    let base_url = reqwest::Url::parse("https://leetcode.com/problems/")?;
    let url = base_url.join(slug)?;

    let content = format!("#[{}. {}]({})\n", qid, title, url);
    let file_path = dir_path.join("README.md");
    fs::write(&file_path, content)?;
    println!("Successfully saved README.md to: {:?}", file_path);
    Ok(())
}

pub fn store_solution(dir_path: &Path, code: &str, lang: Language) -> Result<(), std::io::Error> {
    if !dir_path.exists() {
        fs::create_dir_all(dir_path)?;
    }

    let file_path = dir_path.join(format!("solution.{}", lang.extension()));

    fs::write(&file_path, code)?;
    
    println!("Successfully saved solution to: {:?}", file_path);
    Ok(())
}