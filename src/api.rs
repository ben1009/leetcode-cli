use std::{collections::HashMap, path::Path, sync::Arc};

use anyhow::{Result, anyhow};
use rand::seq::IndexedRandom;
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    problem::{DifficultyLevel, Problem, ProblemDetail, ProblemList},
};

/// LeetCode API client for fetching problems and submitting solutions.
///
/// # Example
///
/// ```ignore
/// use leetcode_cli::api::LeetCodeClient;
/// use leetcode_cli::config::Config;
///
/// #[tokio::main]
/// async fn main() -> anyhow::Result<()> {
///     let config = Config::load()?;
///     let client = LeetCodeClient::new(config).await?;
///     
///     // Get a random easy problem
///     let problem = client.get_random_problem(Some("easy"), None).await?;
///     if let Some(p) = problem {
///         println!("Found problem: {}", p.stat.question_title());
///     }
///     
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct LeetCodeClient {
    client: Client,
    config: Config,
    problems: Arc<Vec<Problem>>,
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
    /// Create a new LeetCode client with the given configuration.
    ///
    /// This will fetch the problem list from LeetCode on initialization.
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
            problems: Arc::new(Vec::new()),
            base_url,
        };

        // Fetch all problems on initialization
        lc_client.fetch_all_problems().await?;

        Ok(lc_client)
    }

    async fn fetch_all_problems(&mut self) -> Result<()> {
        let url = format!("{}/api/problems/all/", self.base_url);
        let response = self.client.get(&url).send().await?;

        if !response.status().is_success() {
            return Err(anyhow!("Failed to fetch problems: {}", response.status()));
        }

        let problem_list: ProblemList = response.json().await?;
        self.problems = Arc::new(problem_list.stat_status_pairs);

        Ok(())
    }

    /// Get all problems as a cheaply cloneable Arc reference.
    ///
    /// Returns an `Arc<Vec<Problem>>` which can be cloned cheaply.
    pub async fn get_all_problems(&self) -> Result<Arc<Vec<Problem>>> {
        Ok(self.problems.clone())
    }

    /// Get a problem by its ID.
    ///
    /// Returns `None` if no problem with the given ID exists.
    pub async fn get_problem_by_id(&self, id: u32) -> Result<Option<Problem>> {
        Ok(self
            .problems
            .iter()
            .find(|p| p.stat.question_id == id)
            .cloned())
    }

    /// Get a random problem, optionally filtered by difficulty and/or tag.
    ///
    /// # Arguments
    ///
    /// * `difficulty` - Optional difficulty filter ("easy", "medium", or "hard")
    /// * `tag` - Optional tag filter (e.g., "array", "dynamic-programming")
    ///
    /// # Note
    ///
    /// Tag filtering requires fetching problem details and is limited to the first 50
    /// matching problems to avoid excessive API calls.
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

        // Filter out paid-only problems
        filtered.retain(|p| !p.paid_only);

        // Filter by tag if specified
        // Note: This requires fetching problem details since the problem list
        // doesn't include tag information. We limit to first 50 to avoid too many API calls.
        if let Some(tag_filter) = tag {
            let tag_slug = tag_filter.to_lowercase().replace(" ", "-");
            let mut tagged_problems = Vec::new();

            for problem in filtered.iter().take(50) {
                match self
                    .get_problem_detail(&problem.stat.question_title_slug())
                    .await
                {
                    Ok(detail) => {
                        if let Some(ref tags) = detail.topic_tags {
                            if tags.iter().any(|t| {
                                t.slug == tag_slug
                                    || t.name.to_lowercase() == tag_filter.to_lowercase()
                            }) {
                                tagged_problems.push(*problem);
                            }
                        }
                    }
                    Err(_) => continue, // Skip problems we can't fetch details for
                }
            }

            if tagged_problems.is_empty() {
                return Ok(None);
            }
            filtered = tagged_problems.to_vec();
        }

        // Pick random problem
        let mut rng = rand::rng();
        Ok(filtered.choose(&mut rng).cloned().cloned())
    }

    /// Get detailed information about a problem by its slug.
    ///
    /// This includes the problem description, examples, code snippets, and tags.
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

        let url = format!("{}/graphql", self.base_url);
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
        let submit_url = format!("{}/problems/{}/submit/", self.base_url, slug);

        // Read solution file
        let code = tokio::fs::read_to_string(solution_file).await?;

        // Extract just the solution code (remove main function and tests if present)
        let cleaned_code = Self::extract_solution_code(&code);

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
            "{}/submissions/detail/{}/check/",
            self.base_url, submission_id
        );

        #[cfg(test)]
        let max_attempts = 2;
        #[cfg(not(test))]
        let max_attempts = 30;

        // Exponential backoff: start at 1s, max 8s
        let mut delay_secs = 1;

        for attempt in 0..max_attempts {
            println!("  Checking result... ({}/{})", attempt + 1, max_attempts);

            let response = self.client.get(&check_url).send().await?;

            if !response.status().is_success() {
                #[cfg(not(test))]
                tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
                // Exponential backoff with cap at 8 seconds
                delay_secs = (delay_secs * 2).min(8);
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

            #[cfg(not(test))]
            tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
            // Exponential backoff with cap at 8 seconds
            delay_secs = (delay_secs * 2).min(8);
        }

        Err(anyhow!("Timeout waiting for submission result"))
    }

    pub(crate) fn extract_solution_code(code: &str) -> String {
        // Find the impl Solution block with proper handling of strings and comments
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

            // Look for impl Solution (but not impl Solution { } in comments)
            if !trimmed.starts_with("//") && trimmed.contains("impl Solution") {
                in_solution = true;
            }

            if in_solution {
                result.push(*line);

                // Count braces, ignoring those in strings and comments
                let delta = count_significant_braces(trimmed, brace_count);
                brace_count = brace_count.wrapping_add_signed(delta);
                if brace_count == 0 && result.len() > 1 {
                    return result.join("\n");
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
}

/// Count braces in a line, ignoring those inside string literals and comments.
/// Returns the net change in brace depth (+1 for each '{', -1 for each '}').
pub(crate) fn count_significant_braces(line: &str, current_depth: usize) -> isize {
    let mut in_string = false;
    let mut in_char = false;
    let mut escape_next = false;
    let mut in_line_comment = false;
    let mut brace_delta: isize = 0;

    for (i, c) in line.chars().enumerate() {
        // Check for line comment start (but not inside strings)
        if !in_string
            && !in_char
            && !in_line_comment
            && c == '/'
            && line.get(i + 1..i + 2) == Some("/")
        {
            in_line_comment = true;
            continue;
        }

        if in_line_comment {
            continue;
        }

        if escape_next {
            escape_next = false;
            continue;
        }

        match c {
            '\\' if in_string || in_char => {
                escape_next = true;
            }
            '"' if !in_char => {
                in_string = !in_string;
            }
            '\'' if !in_string => {
                // Handle char literals, being careful about lifetime syntax like 'a
                if !in_char {
                    // Check if this looks like a lifetime
                    let prev = i.checked_sub(1).and_then(|j| line.chars().nth(j));
                    let is_lifetime = prev.is_some_and(|p| p.is_alphanumeric() || p == '_');
                    if !is_lifetime {
                        in_char = true;
                    }
                } else {
                    in_char = false;
                }
            }
            '{' if !in_string && !in_char => {
                brace_delta += 1;
            }
            '}' if !in_string && !in_char => {
                // Don't go below zero at the line level
                if current_depth.wrapping_add_signed(brace_delta) > 0 {
                    brace_delta -= 1;
                }
            }
            _ => {}
        }
    }

    brace_delta
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
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
        assert_eq!(problem.as_ref().unwrap().stat.question_id, 1);

        let problem = client.get_problem_by_id(999).await.unwrap();
        assert!(problem.is_none());
    }

    #[tokio::test]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
        assert_eq!(problem.as_ref().unwrap().difficulty.level, 1);

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
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_get_random_problem_with_tag() {
        let (mock_server, config) = setup_mock_server().await;

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(create_test_problem_list()))
            .mount(&mock_server)
            .await;

        // Mock GraphQL for two-sum with array tag
        let two_sum_graphql = serde_json::json!({
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
                    "codeSnippets": [],
                    "hints": [],
                    "topicTags": [{"name": "Array", "slug": "array"}]
                }
            }
        });

        // Mock GraphQL for add-two-numbers with linked-list tag
        let add_two_numbers_graphql = serde_json::json!({
            "data": {
                "question": {
                    "questionId": "2",
                    "title": "Add Two Numbers",
                    "titleSlug": "add-two-numbers",
                    "content": "<p>Add two numbers...</p>",
                    "difficulty": "Medium",
                    "exampleTestcases": "[2,4,3]\\n[5,6,4]",
                    "sampleTestCase": "[2,4,3]\\n[5,6,4]",
                    "metaData": null,
                    "codeSnippets": [],
                    "hints": [],
                    "topicTags": [{"name": "Linked List", "slug": "linked-list"}]
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .and(wiremock::matchers::body_string_contains("two-sum"))
            .respond_with(ResponseTemplate::new(200).set_body_json(two_sum_graphql))
            .mount(&mock_server)
            .await;

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .and(wiremock::matchers::body_string_contains("add-two-numbers"))
            .respond_with(ResponseTemplate::new(200).set_body_json(add_two_numbers_graphql))
            .mount(&mock_server)
            .await;

        let client = LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        // Test with array tag - should find Two Sum
        let problem = client
            .get_random_problem(None, Some("array"))
            .await
            .unwrap();
        assert!(problem.is_some());
        assert_eq!(problem.as_ref().unwrap().stat.question_id, 1);

        // Test with linked-list tag - should find Add Two Numbers
        let problem = client
            .get_random_problem(None, Some("linked-list"))
            .await
            .unwrap();
        assert!(problem.is_some());
        assert_eq!(problem.as_ref().unwrap().stat.question_id, 2);

        // Test with non-existent tag
        let problem = client
            .get_random_problem(None, Some("non-existent-tag"))
            .await
            .unwrap();
        assert!(problem.is_none());
    }

    #[tokio::test]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
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
        let extracted = LeetCodeClient::extract_solution_code(code);
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

        let extracted = LeetCodeClient::extract_solution_code(code);
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

        let extracted = LeetCodeClient::extract_solution_code(code);
        assert!(extracted.contains("impl Solution"));
        assert!(extracted.contains("match Some(1)"));
        assert!(!extracted.contains("fn main()"));
    }

    #[test]
    fn test_extract_solution_code_braces_in_strings() {
        // Test that braces inside string literals don't affect extraction
        let code = r#"impl Solution {
    pub fn test() -> String {
        let s = "This has { braces } and } more";
        let t = "Another } brace";
        format!("{}", s)
    }
}

fn main() {}"#;

        let extracted = LeetCodeClient::extract_solution_code(code);
        assert!(extracted.contains("impl Solution"));
        assert!(extracted.contains(r#""This has { braces } and } more""#));
        assert!(extracted.contains("format!"));
        assert!(!extracted.contains("fn main()"));
    }

    #[test]
    fn test_extract_solution_code_braces_in_comments() {
        // Test that braces inside comments don't affect extraction
        let code = r#"impl Solution {
    pub fn test() {
        // This comment has { braces }
        /* Block comment with } */
        let x = 1;
    }
}

fn main() {}"#;

        let extracted = LeetCodeClient::extract_solution_code(code);
        assert!(extracted.contains("impl Solution"));
        assert!(extracted.contains("let x = 1"));
        assert!(!extracted.contains("fn main()"));
    }

    #[test]
    fn test_extract_solution_code_only_main() {
        // Code with only helper function and main, no impl Solution
        let code = r#"fn helper() {
    println!("Hello");
}

fn main() {
    helper();
}

#[cfg(test)]
mod tests {}"#;

        let extracted = LeetCodeClient::extract_solution_code(code);
        // When there's no impl Solution, it returns code until main() or tests
        assert!(extracted.contains("fn helper()"));
        assert!(!extracted.contains("fn main()"));
        assert!(!extracted.contains("mod tests"));
    }

    #[test]
    fn test_extract_solution_code_lifetimes() {
        // Code with lifetime annotations (shouldn't be confused with char literals)
        let code = r#"impl Solution {
    pub fn test<'a>(x: &'a str) -> &'a str {
        x
    }
}

fn main() {}"#;

        let extracted = LeetCodeClient::extract_solution_code(code);
        assert!(extracted.contains("impl Solution"));
        assert!(extracted.contains("test<'a>"));
        assert!(!extracted.contains("fn main()"));
    }

    #[test]
    fn test_extract_solution_code_escaped_quotes() {
        // Code with escaped quotes inside strings
        let code = r#"impl Solution {
    pub fn test() -> String {
        let s = "This has \\"escaped quotes\\"";
        s.to_string()
    }
}

fn main() {}"#;

        let extracted = LeetCodeClient::extract_solution_code(code);
        assert!(extracted.contains("impl Solution"));
        assert!(extracted.contains(r#"\\"escaped quotes\\""#));
        assert!(!extracted.contains("fn main()"));
    }

    #[test]
    fn test_count_significant_braces_basic() {
        // Directly test brace counting
        assert_eq!(count_significant_braces("{", 0), 1);
        assert_eq!(count_significant_braces("}", 1), -1);
        assert_eq!(count_significant_braces("{}", 0), 0);
        assert_eq!(count_significant_braces("{{}}", 0), 0);
    }

    #[test]
    fn test_count_significant_braces_in_strings() {
        // Braces in strings should be ignored
        assert_eq!(count_significant_braces(r#""{""#, 0), 0);
        assert_eq!(count_significant_braces(r#""} {""#, 1), 0);
        assert_eq!(count_significant_braces(r#""{""#, 0), 0);
        assert_eq!(count_significant_braces(r#""}""#, 1), 0);
    }

    #[test]
    fn test_count_significant_braces_in_comments() {
        // Braces in comments should be ignored
        assert_eq!(count_significant_braces("// { }", 0), 0);
        assert_eq!(count_significant_braces("code // { }", 0), 0);
        assert_eq!(count_significant_braces("code { // }", 0), 1);
    }

    #[test]
    fn test_count_significant_braces_respects_depth() {
        // Brace closing respects current depth
        assert_eq!(count_significant_braces("}", 0), 0); // Can't go below 0
        assert_eq!(count_significant_braces("}", 1), -1); // Can decrease at depth 1
        assert_eq!(count_significant_braces("}}", 1), -1); // Only decreases by 1
    }

    #[test]
    fn test_count_significant_braces_char_literals() {
        // Braces in char literals should be ignored
        assert_eq!(count_significant_braces("'{'", 0), 0);
        assert_eq!(count_significant_braces("'}'", 1), 0);
    }

    #[test]
    fn test_count_significant_braces_lifetimes() {
        // Lifetime annotations shouldn't be confused with char literals
        assert_eq!(count_significant_braces("<'a>", 0), 0);
        assert_eq!(count_significant_braces("<'a, 'b>", 0), 0);
    }
}
