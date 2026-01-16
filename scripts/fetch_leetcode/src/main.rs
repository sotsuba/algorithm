use reqwest::header::{self, HeaderMap, HeaderValue, HeaderName};

fn main() {
    let mut headers = HeaderMap::new();
    headers.insert(header::USER_AGENT, HeaderValue::from_static("Mozilla/5.0"));
    headers.insert(header::REFERER, HeaderValue::from_static("https://leetcode.com/"));
    println!("{headers:#?}");

    let client = reqwest::Client::new();
}
