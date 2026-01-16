mod models;
mod utils;
mod client;
use std::fs;
use std::env;
use models::Language;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let session = env::var("LEETCODE_SESSION")?;
    let csrf_token = env::var("LEETCODE_CSRF")?;
    let user_agent = env::var("USER_AGENT")?;
    
    let client = client::Client::new(&session, &csrf_token, &user_agent).await?;
    let code = client.get_latest_accepted_code(Language::Cpp).await?;
    let code = client.get_latest_accepted_code(Language::Rust).await?;
    let code = client.get_latest_accepted_code(Language::Python).await?;
    Ok(())
}

