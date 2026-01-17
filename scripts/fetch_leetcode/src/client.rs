use reqwest::header::{self, HeaderValue, HeaderMap};
use reqwest::cookie::Jar;

use std::error::Error;
use std::sync::Arc;
use serde_json::json;
use std::path::PathBuf;

use crate::models::{SyncResult, QuestionMetadata, DailyResponse, Submission, SubmissionListResponse, SubmissionDetailResponse, Language};
use crate::utils::{get_problem_dir, store_solution, create_readme};
pub struct Client { 
    inner: reqwest::Client,
    base_url: reqwest::Url,
}

impl Client {
    pub async fn new(session: &str, csrf_token: &str, user_agent: &str) -> Result<Self, Box<dyn Error>> {
        let base_url = "https://leetcode.com/graphql".parse::<reqwest::Url>()?; //
        let jar = Arc::new(Jar::default()); //

        jar.add_cookie_str(&format!("LEETCODE_SESSION={}", session), &base_url);
        jar.add_cookie_str(&format!("csrftoken={}", csrf_token), &base_url);

        let mut headers = HeaderMap::new();
        headers.insert("x-csrftoken", HeaderValue::from_str(&csrf_token)?); 
        headers.insert(header::USER_AGENT, HeaderValue::from_str(&user_agent)?);

        let inner = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&jar)) //
            .default_headers(headers)
            .build()?;
        
        Ok(Self { inner, base_url }) 
    }

    pub async fn get_latest_accepted_code(&self, target_lang: Language) -> Result<Option<SyncResult>, Box<dyn Error>> {
        let metadata = self.get_daily_challenge().await?;
        let base_path = get_problem_dir(&metadata)?;
        let submissions = self.get_submission_list(&metadata.title_slug).await?;

        let accepted_id = submissions.iter()
            .find(|s| s.status_display == "Accepted" && s.lang == target_lang)
            .map(|s| s.id.clone());

        match accepted_id {
            Some(id) => {
                let code = self.get_submission_details(id).await?;
                let path = get_problem_dir(&metadata)?;
                store_solution(&path, &code, target_lang);
                create_readme(&path, &metadata);
                Ok(Some(SyncResult{ code, path }))   
            },
            None => Ok(None),
        }
    }



    async fn get_daily_challenge(&self) -> Result<QuestionMetadata, Box<dyn std::error::Error>> {
        let query = serde_json::json!({
            "query": "query questionOfToday { activeDailyCodingChallengeQuestion { question { questionFrontendId title titleSlug } } }"
        });

        // DEBUG STEP: Use .text() if you keep getting the Decode error to see the HTML
        let resp = self.inner.post(self.base_url.clone())
            .json(&query)
            .send()
            .await?;
            
        // Critical: Check for 403/404 before decoding
        if !resp.status().is_success() {
            return Err(format!("API Error: Status {}", resp.status()).into());
        }

        let data: DailyResponse = resp.json().await?;
        Ok(data.data.active_daily_coding_challenge_question.question)
    }

    async fn get_submission_list(&self, slug: &str) -> Result<Vec<Submission>, Box<dyn Error>> {
        let query = json!({
            "query": "
                query questionSubmissionList($questionSlug: String!, $offset: Int, $limit: Int) {
                    questionSubmissionList(questionSlug: $questionSlug, offset: $offset, limit: $limit) {
                        submissions {
                            id
                            statusDisplay
                            lang
                        }
                    }
                }
            ",
            "variables": {
                "questionSlug": slug,
                "offset": 0,
                "limit": 20
            }
        });

        let resp: SubmissionListResponse = self.inner 
            .post(self.base_url.clone())
            .json(&query)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.data.question_submission_list.submissions)
    }

    async fn get_submission_details(&self, id: String) -> Result<String, Box<dyn Error>> {
        let query = json!({
            "query": "
                query submissionDetails($submissionId: Int!) {
                    submissionDetails(submissionId: $submissionId) {
                        code
                    }
                }
            ",
            "variables": {
                "submissionId": id.parse::<i32>()? 
            }
        });

        let resp: SubmissionDetailResponse = self.inner
            .post(self.base_url.clone())
            .json(&query)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.data.submission_details.code)
    }
}