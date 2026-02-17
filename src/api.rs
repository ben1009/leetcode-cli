use anyhow::{anyhow, Result};
use rand::seq::SliceRandom;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::config::Config;
use crate::problem::{Problem, ProblemDetail, ProblemList};

const LEETCODE_API_URL: &str = "https://leetcode.com/api/problems/all/";
const LEETCODE_GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const LEETCODE_SUBMIT_URL: &str = "https://leetcode.com/problems/{}/submit/";

#[derive(Debug, Clone)]
pub struct LeetCodeClient {
    client: Client,
    config: Config,
    problems: Vec<Problem>,
}

#[derive(Debug, Deserialize)]
pub struct SubmissionResult {
    pub status_code: i32,
    pub status_msg: String,
    pub status_runtime: String,
    pub status_memory: String,
    pub runtime_percentile: f64,
    pub memory_percentile: f64,
    pub code_output: Option<String>,
    pub expected_output: Option<String>,
    pub full_runtime_error: Option<String>,
    pub full_compile_error: Option<String>,
    pub total_correct: Option<i32>,
    pub total_testcases: Option<i32>,
    pub input_formatted: Option<String>,
}

#[derive(Debug, Serialize)]
struct GraphQLQuery {
    query: String,
    variables: HashMap<String, serde_json::Value>,
}

impl LeetCodeClient {
    pub async fn new(config: Config) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            ),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::REFERER,
            header::HeaderValue::from_static("https://leetcode.com/"),
        );

        // Add authentication cookies if available
        if let Some(ref session) = config.session_cookie {
            let cookie_value = format!("LEETCODE_SESSION={}", session);
            headers.insert(
                header::COOKIE,
                header::HeaderValue::from_str(&cookie_value)?,
            );
        }

        if let Some(ref csrf) = config.csrf_token {
            headers.insert(
                header::HeaderName::from_static("x-csrftoken"),
                header::HeaderValue::from_str(csrf)?,
            );
        }

        let client = Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()?;

        let mut lc_client = Self {
            client,
            config,
            problems: Vec::new(),
        };

        // Fetch all problems on initialization
        lc_client.fetch_all_problems().await?;

        Ok(lc_client)
    }

    async fn fetch_all_problems(&mut self) -> Result<()> {
        let response = self.client.get(LEETCODE_API_URL).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch problems: {}", response.status()));
        }

        let problem_list: ProblemList = response.json().await?;
        self.problems = problem_list.stat_status_pairs;

        Ok(())
    }

    pub async fn get_all_problems(&self) -> Result<Vec<Problem>> {
        Ok(self.problems.clone())
    }

    pub async fn get_problem_by_id(&self, id: u32) -> Result<Option<Problem>> {
        Ok(self
            .problems
            .iter()
            .find(|p| p.stat.question_id == id)
            .cloned())
    }

    pub async fn get_random_problem(
        &self,
        difficulty: Option<&str>,
        tag: Option<&str>,
    ) -> Result<Option<Problem>> {
        let mut filtered: Vec<&Problem> = self.problems.iter().collect();

        // Filter by difficulty
        if let Some(diff) = difficulty {
            let level = match diff.to_lowercase().as_str() {
                "easy" => 1,
                "medium" => 2,
                "hard" => 3,
                _ => 0,
            };
            if level > 0 {
                filtered.retain(|p| p.difficulty.level == level);
            }
        }

        // Filter by tag (simplified - in real implementation, you'd need to fetch tags)
        if let Some(_t) = tag {
            // This would require additional API calls to filter by tag
            // For now, we'll skip this filter
        }

        // Filter out paid-only problems
        filtered.retain(|p| !p.paid_only);

        // Pick random problem
        let mut rng = rand::thread_rng();
        Ok(filtered.choose(&mut rng).cloned().cloned())
    }

    pub async fn get_problem_detail(&self, slug: &str) -> Result<ProblemDetail> {
        let query = GraphQLQuery {
            query: r#"
                query getQuestionDetail($titleSlug: String!) {
                    question(titleSlug: $titleSlug) {
                        questionId
                        title
                        titleSlug
                        content
                        difficulty
                        exampleTestcases
                        sampleTestCase
                        metaData
                        codeSnippets {
                            lang
                            langSlug
                            code
                        }
                        hints
                        topicTags {
                            name
                            slug
                        }
                    }
                }
            "#
            .to_string(),
            variables: {
                let mut map = HashMap::new();
                map.insert("titleSlug".to_string(), serde_json::json!(slug));
                map
            },
        };

        let response = self
            .client
            .post(LEETCODE_GRAPHQL_URL)
            .json(&query)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!(
                "Failed to fetch problem detail: {}",
                response.status()
            ));
        }

        let result: serde_json::Value = response.json().await?;

        let question = result
            .get("data")
            .and_then(|d| d.get("question"))
            .ok_or_else(|| anyhow!("Invalid response format"))?;

        let detail: ProblemDetail = serde_json::from_value(question.clone())?;
        Ok(detail)
    }

    pub async fn submit(&self, problem_id: u32, solution_file: &Path) -> Result<SubmissionResult> {
        // Check if authenticated
        if self.config.session_cookie.is_none() {
            return Err(anyhow!(
                "Not authenticated. Please run 'leetcode-cli login' first."
            ));
        }

        let problem = self
            .get_problem_by_id(problem_id)
            .await?
            .ok_or_else(|| anyhow!("Problem not found"))?;

        let slug = &problem.stat.question_title_slug();
        let submit_url = LEETCODE_SUBMIT_URL.replace("{}", slug);

        // Read solution file
        let code = tokio::fs::read_to_string(solution_file).await?;

        // Extract just the solution code (remove main function and tests if present)
        let cleaned_code = self.extract_solution_code(&code);

        let payload = serde_json::json!({
            "lang": "rust",
            "question_id": problem_id.to_string(),
            "typed_code": cleaned_code,
        });

        let response = self.client.post(&submit_url).json(&payload).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to submit: {}", response.status()));
        }

        let submit_response: serde_json::Value = response.json().await?;
        let submission_id = submit_response
            .get("submission_id")
            .and_then(|id| id.as_i64())
            .ok_or_else(|| anyhow!("Failed to get submission ID"))?;

        // Poll for result
        self.poll_submission_result(submission_id).await
    }

    async fn poll_submission_result(&self, submission_id: i64) -> Result<SubmissionResult> {
        let check_url = format!(
            "https://leetcode.com/submissions/detail/{}/check/",
            submission_id
        );

        let max_attempts = 30;
        let delay_secs = 2;

        for attempt in 0..max_attempts {
            println!("  Checking result... ({}/{})", attempt + 1, max_attempts);

            let response = self.client.get(&check_url).send().await?;

            if !response.status().is_success() {
                tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
                continue;
            }

            let result: serde_json::Value = response.json().await?;

            // Check if submission is complete
            if let Some(state) = result.get("state").and_then(|s| s.as_str()) {
                if state == "SUCCESS" {
                    let submission_result: SubmissionResult = serde_json::from_value(result)?;
                    return Ok(submission_result);
                }
            }

            tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
        }

        Err(anyhow!("Timeout waiting for submission result"))
    }

    fn extract_solution_code(&self, code: &str) -> String {
        // Simple extraction - find the impl Solution block
        let lines: Vec<&str> = code.lines().collect();
        let mut result = Vec::new();
        let mut in_solution = false;
        let mut brace_count = 0;

        for line in &lines {
            let trimmed = line.trim();

            // Skip main function and test modules
            if trimmed.starts_with("fn main()") || trimmed.starts_with("#[cfg(test)]") {
                break;
            }

            // Look for impl Solution
            if trimmed.contains("impl Solution") {
                in_solution = true;
            }

            if in_solution {
                result.push(*line);

                // Count braces to find end of impl block
                for c in trimmed.chars() {
                    if c == '{' {
                        brace_count += 1;
                    } else if c == '}' {
                        brace_count -= 1;
                        if brace_count == 0 {
                            return result.join("\n");
                        }
                    }
                }
            }
        }

        // If we couldn't extract properly, return the whole code
        // but try to remove main and tests
        code.lines()
            .take_while(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("fn main()") && !trimmed.starts_with("#[cfg(test)]")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub async fn check_solution_status(&self, submission_id: i64) -> Result<SubmissionResult> {
        let check_url = format!(
            "https://leetcode.com/submissions/detail/{}/check/",
            submission_id
        );

        let response = self.client.get(&check_url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to check status: {}", response.status()));
        }

        let result: SubmissionResult = response.json().await?;
        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
struct SubmitResponse {
    #[serde(rename = "submission_id")]
    submission_id: i64,
}
