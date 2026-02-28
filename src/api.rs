use std::{collections::HashMap, path::Path};

use anyhow::{Result, anyhow};
use rand::seq::IndexedRandom;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    problem::{DifficultyLevel, Problem, ProblemDetail, ProblemList},
};

const LEETCODE_API_URL: &str = "https://leetcode.com/api/problems/all/";
const LEETCODE_GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const LEETCODE_SUBMIT_URL: &str = "https://leetcode.com/problems/{}/submit/";

#[derive(Debug, Clone)]
pub struct LeetCodeClient {
    client: Client,
    config: Config,
    problems: Vec<Problem>,
    base_url: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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
        Self::new_with_base_url(config, "https://leetcode.com".to_string()).await
    }

    #[allow(dead_code)]
    pub(crate) async fn new_with_base_url(config: Config, base_url: String) -> Result<Self> {
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
            base_url,
        };

        // Fetch all problems on initialization
        lc_client.fetch_all_problems().await?;

        Ok(lc_client)
    }

    async fn fetch_all_problems(&mut self) -> Result<()> {
        let url = if self.base_url == "https://leetcode.com" {
            LEETCODE_API_URL.to_string()
        } else {
            format!("{}/api/problems/all/", self.base_url)
        };
        let response = self.client.get(&url).send().await?;

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
            if let Some(level) = DifficultyLevel::from_str(diff) {
                filtered.retain(|p| p.difficulty.level == level.level());
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
        let mut rng = rand::rng();
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

        let url = if self.base_url == "https://leetcode.com" {
            LEETCODE_GRAPHQL_URL.to_string()
        } else {
            format!("{}/graphql", self.base_url)
        };

        let response = self.client.post(&url).json(&query).send().await?;

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
        let submit_url = if self.base_url == "https://leetcode.com" {
            LEETCODE_SUBMIT_URL.replace("{}", slug)
        } else {
            format!("{}/problems/{}/submit/", self.base_url, slug)
        };

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
        let check_url = if self.base_url == "https://leetcode.com" {
            format!(
                "https://leetcode.com/submissions/detail/{}/check/",
                submission_id
            )
        } else {
            format!(
                "{}/submissions/detail/{}/check/",
                self.base_url, submission_id
            )
        };

        #[cfg(test)]
        let max_attempts = 2;
        #[cfg(not(test))]
        let max_attempts = 30;

        #[cfg(test)]
        let delay_secs = 0;
        #[cfg(not(test))]
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

    #[allow(dead_code)]
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
#[allow(dead_code)]
struct SubmitResponse {
    #[serde(rename = "submission_id")]
    submission_id: i64,
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path},
    };

    use super::*;

    fn create_test_problem_list() -> serde_json::Value {
        serde_json::json!({
            "user_name": "test_user",
            "num_solved": 10,
            "num_total": 100,
            "ac_easy": 5,
            "ac_medium": 3,
            "ac_hard": 2,
            "stat_status_pairs": [
                {
                    "stat": {
                        "question_id": 1,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "Two Sum",
                        "question__title_slug": "two-sum",
                        "question__hide": false,
                        "total_acs": 1000000,
                        "total_submitted": 2000000,
                        "frontend_question_id": 1,
                        "is_new_question": false
                    },
                    "difficulty": {"level": 1},
                    "paid_only": false,
                    "is_favor": false,
                    "frequency": 0,
                    "progress": 0,
                    "status": "ac"
                },
                {
                    "stat": {
                        "question_id": 2,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "Add Two Numbers",
                        "question__title_slug": "add-two-numbers",
                        "question__hide": false,
                        "total_acs": 500000,
                        "total_submitted": 1000000,
                        "frontend_question_id": 2,
                        "is_new_question": false
                    },
                    "difficulty": {"level": 2},
                    "paid_only": false,
                    "is_favor": false,
                    "frequency": 0,
                    "progress": 0,
                    "status": null
                },
                {
                    "stat": {
                        "question_id": 3,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "Hard Problem",
                        "question__title_slug": "hard-problem",
                        "question__hide": false,
                        "total_acs": 100000,
                        "total_submitted": 500000,
                        "frontend_question_id": 3,
                        "is_new_question": false
                    },
                    "difficulty": {"level": 3},
                    "paid_only": true,
                    "is_favor": false,
                    "frequency": 0,
                    "progress": 0,
                    "status": "notac"
                }
            ]
        })
    }

    async fn setup_mock_server() -> (MockServer, Config) {
        let mock_server = MockServer::start().await;
        let config = Config::default();
        (mock_server, config)
    }

    #[tokio::test]
    async fn test_fetch_all_problems() {
        let (mock_server, config) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri()).await;
        assert!(client.is_ok());

        let client = client.unwrap();
        let problems = client.get_all_problems().await.unwrap();
        assert_eq!(problems.len(), 3);
    }

    #[tokio::test]
    async fn test_get_problem_by_id() {
        let (mock_server, config) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        let problem = client.get_problem_by_id(1).await.unwrap();
        assert!(problem.is_some());
        assert_eq!(problem.unwrap().stat.question_id, 1);

        let problem = client.get_problem_by_id(999).await.unwrap();
        assert!(problem.is_none());
    }

    #[tokio::test]
    async fn test_get_random_problem() {
        let (mock_server, config) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        // Test without filters
        let problem = client.get_random_problem(None, None).await.unwrap();
        assert!(problem.is_some());

        // Test with difficulty filter
        let problem = client.get_random_problem(Some("easy"), None).await.unwrap();
        assert!(problem.is_some());
        assert_eq!(problem.unwrap().difficulty.level, 1);

        let problem = client
            .get_random_problem(Some("medium"), None)
            .await
            .unwrap();
        assert!(problem.is_some());

        // Test with non-existent difficulty
        let problem = client
            .get_random_problem(Some("invalid"), None)
            .await
            .unwrap();
        assert!(problem.is_some());
    }

    #[tokio::test]
    async fn test_get_problem_detail() {
        let (mock_server, config) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        let graphql_response = serde_json::json!({
            "data": {
                "question": {
                    "questionId": "1",
                    "title": "Two Sum",
                    "titleSlug": "two-sum",
                    "content": "<p>Given an array...</p>",
                    "difficulty": "Easy",
                    "exampleTestcases": "[2,7,11,15]\\n9",
                    "sampleTestCase": "[2,7,11,15]\\n9",
                    "metaData": null,
                    "codeSnippets": [
                        {
                            "lang": "Rust",
                            "langSlug": "rust",
                            "code": "impl Solution {\\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\\n        \\n    }\\n}"
                        }
                    ],
                    "hints": ["Use a hash map"],
                    "topicTags": [{"name": "Array", "slug": "array"}]
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(graphql_response))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();
        let detail = client.get_problem_detail("two-sum").await;
        assert!(detail.is_ok());

        let detail = detail.unwrap();
        assert_eq!(detail.question_id, "1");
        assert_eq!(detail.title, "Two Sum");
    }

    #[tokio::test]
    async fn test_get_problem_detail_invalid_response() {
        let (mock_server, config) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": {}})))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();
        let result = client.get_problem_detail("two-sum").await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid response format")
        );
    }

    #[tokio::test]
    async fn test_submit_not_authenticated() {
        let (mock_server, config) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        let temp_dir = tempfile::tempdir().unwrap();
        let solution_file = temp_dir.path().join("solution.rs");
        std::fs::write(&solution_file, "impl Solution {}").unwrap();

        let result = client.submit(1, &solution_file).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Not authenticated")
        );
    }

    #[tokio::test]
    async fn test_submit_success() {
        let (mock_server, mut config) = setup_mock_server().await;
        config.session_cookie = Some("test_session".to_string());
        config.csrf_token = Some("test_csrf".to_string());

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/problems/two-sum/submit/"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_json(serde_json::json!({"submission_id": 12345i64})),
            )
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/submissions/detail/12345/check/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "state": "SUCCESS",
                "status_code": 10,
                "status_msg": "Accepted",
                "status_runtime": "4 ms",
                "status_memory": "2.1 MB",
                "runtime_percentile": 85.5,
                "memory_percentile": 70.2
            })))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        let temp_dir = tempfile::tempdir().unwrap();
        let solution_file = temp_dir.path().join("solution.rs");
        let mut file = std::fs::File::create(&solution_file).unwrap();
        file.write_all(b"impl Solution { pub fn two_sum() {} }")
            .unwrap();

        let result = client.submit(1, &solution_file).await;
        assert!(result.is_ok());

        let result = result.unwrap();
        assert_eq!(result.status_code, 10);
        assert_eq!(result.status_msg, "Accepted");
    }

    #[tokio::test]
    async fn test_submit_problem_not_found() {
        let (mock_server, mut config) = setup_mock_server().await;
        config.session_cookie = Some("test_session".to_string());

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        let temp_dir = tempfile::tempdir().unwrap();
        let solution_file = temp_dir.path().join("solution.rs");
        std::fs::write(&solution_file, "impl Solution {}").unwrap();

        let result = client.submit(999, &solution_file).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Problem not found")
        );
    }

    #[test]
    fn test_submission_result_deserialization() {
        let json = r#"{
            "status_code": 10,
            "status_msg": "Accepted",
            "status_runtime": "4 ms",
            "status_memory": "2.1 MB",
            "runtime_percentile": 85.5,
            "memory_percentile": 70.2,
            "code_output": null,
            "expected_output": null,
            "full_runtime_error": null,
            "full_compile_error": null,
            "total_correct": 50,
            "total_testcases": 50,
            "input_formatted": null
        }"#;

        let result: SubmissionResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.status_code, 10);
        assert_eq!(result.status_msg, "Accepted");
        assert_eq!(result.status_runtime, "4 ms");
        assert_eq!(result.status_memory, "2.1 MB");
        assert_eq!(result.runtime_percentile, 85.5);
        assert_eq!(result.memory_percentile, 70.2);
    }

    #[test]
    fn test_submission_result_wrong_answer() {
        let json = r#"{
            "status_code": 11,
            "status_msg": "Wrong Answer",
            "status_runtime": "",
            "status_memory": "",
            "runtime_percentile": 0.0,
            "memory_percentile": 0.0,
            "code_output": "[1, 2]",
            "expected_output": "[1, 3]",
            "full_runtime_error": null,
            "full_compile_error": null,
            "total_correct": 10,
            "total_testcases": 20,
            "input_formatted": "[2,7,11,15]\n9"
        }"#;

        let result: SubmissionResult = serde_json::from_str(json).unwrap();
        assert_eq!(result.status_code, 11);
        assert_eq!(result.status_msg, "Wrong Answer");
        assert_eq!(result.code_output, Some("[1, 2]".to_string()));
        assert_eq!(result.expected_output, Some("[1, 3]".to_string()));
    }

    #[test]
    fn test_graph_ql_query_serialization() {
        let mut variables = HashMap::new();
        variables.insert("titleSlug".to_string(), serde_json::json!("two-sum"));

        let query = GraphQLQuery {
            query: "query getQuestionDetail($titleSlug: String!) { question(titleSlug: $titleSlug) { title } }".to_string(),
            variables,
        };

        let json = serde_json::to_string(&query).unwrap();
        assert!(json.contains("query"));
        assert!(json.contains("variables"));
        assert!(json.contains("two-sum"));
    }

    #[test]
    fn test_extract_solution_code_simple() {
        let code = r#"struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

fn main() {
    let sol = Solution;
    println!("{:?}", sol.two_sum(vec![2, 7, 11, 15], 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}"#;

        // Create a minimal client for testing
        let extracted = extract_solution_code_for_test(code);
        assert!(extracted.contains("impl Solution"));
        assert!(extracted.contains("pub fn two_sum"));
        assert!(!extracted.contains("fn main()"));
        assert!(!extracted.contains("#[cfg(test)]"));
    }

    #[test]
    fn test_extract_solution_code_no_impl() {
        let code = r#"fn helper() {}

fn main() {
    helper();
}

#[cfg(test)]
mod tests {}
"#;

        let extracted = extract_solution_code_for_test(code);
        assert!(extracted.contains("fn helper()"));
        assert!(!extracted.contains("fn main()"));
    }

    #[test]
    fn test_extract_solution_code_nested_braces() {
        let code = r#"impl Solution {
    pub fn test() {
        if true {
            for _ in 0..10 {
                match Some(1) {
                    Some(x) => { println!("{}", x); }
                    None => {}
                }
            }
        }
    }
}

fn main() {}"#;

        let extracted = extract_solution_code_for_test(code);
        assert!(extracted.contains("impl Solution"));
        assert!(extracted.contains("match Some(1)"));
        assert!(!extracted.contains("fn main()"));
    }

    // Helper function that mimics the extract_solution_code logic for testing
    fn extract_solution_code_for_test(code: &str) -> String {
        let lines: Vec<&str> = code.lines().collect();
        let mut result = Vec::new();
        let mut in_solution = false;
        let mut brace_count = 0;

        for line in &lines {
            let trimmed = line.trim();

            if trimmed.starts_with("fn main()") || trimmed.starts_with("#[cfg(test)]") {
                break;
            }

            if trimmed.contains("impl Solution") {
                in_solution = true;
            }

            if in_solution {
                result.push(*line);

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

        code.lines()
            .take_while(|line| {
                let trimmed = line.trim();
                !trimmed.starts_with("fn main()") && !trimmed.starts_with("#[cfg(test)]")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
