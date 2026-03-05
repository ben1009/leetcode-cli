//! Show command - Display problem details

use anyhow::Result;
use colored::Colorize;

use crate::{api::LeetCodeClient, problem::DifficultyLevel};

/// Show problem details
pub async fn execute(client: &LeetCodeClient, id: u32) -> Result<()> {
    let problem = client
        .get_problem_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("problem not found: ID {id}"))?;

    let detail = client
        .get_problem_detail(&problem.stat.question_title_slug())
        .await?;

    println!("\n{}", "═".repeat(80).cyan());
    println!(
        "{} {}. {}",
        "Problem".bold(),
        problem.stat.question_id,
        problem.stat.question_title().bold()
    );
    println!("{}", "═".repeat(80).cyan());

    let diff_str = match DifficultyLevel::try_from(problem.difficulty.level) {
        Ok(DifficultyLevel::Easy) => "Easy".green(),
        Ok(DifficultyLevel::Medium) => "Medium".yellow(),
        Ok(DifficultyLevel::Hard) => "Hard".red(),
        Err(_) => "Unknown".normal(),
    };
    println!("{} {}", "Difficulty:".bold(), diff_str);
    println!(
        "{} {:.1}%",
        "Acceptance Rate:".bold(),
        problem.stat.total_acs as f64 / problem.stat.total_submitted as f64 * 100.0
    );
    println!("{}", "─".repeat(80).cyan());

    // Print description
    println!("\n{}", detail.clean_content());

    // Print examples if available
    if let Some(examples) = &detail.example_testcases {
        println!("{}", "Examples:".bold());
        for (i, example) in examples.lines().enumerate() {
            println!("  {} {}", format!("{}.", i + 1).cyan(), example);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::{CodeSnippet, Difficulty, Problem, ProblemDetail, Stat};

    fn create_test_problem(id: u32, title: &str, slug: &str, level: i32) -> Problem {
        Problem {
            stat: Stat {
                question_id: id,
                question__article__live: None,
                question__article__slug: None,
                question__title: Some(title.to_string()),
                question__title_slug: slug.to_string(),
                question__hide: false,
                total_acs: 1000,
                total_submitted: 2000,
                frontend_question_id: id,
                is_new_question: false,
            },
            difficulty: Difficulty { level },
            paid_only: false,
            is_favor: false,
            frequency: 0,
            progress: 0,
            status: None,
        }
    }

    fn create_test_problem_detail(id: &str, title: &str, slug: &str) -> ProblemDetail {
        ProblemDetail {
            question_id: id.to_string(),
            title: title.to_string(),
            title_slug: slug.to_string(),
            content: "<p>Test problem description with <strong>bold</strong> text.</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: Some("input1\noutput1\n\ninput2\noutput2".to_string()),
            sample_test_case: Some("sample".to_string()),
            meta_data: None,
            code_snippets: Some(vec![CodeSnippet {
                lang: "Rust".to_string(),
                lang_slug: "rust".to_string(),
                code: "impl Solution { pub fn solve() {} }".to_string(),
            }]),
            hints: Some(vec!["Hint 1".to_string(), "Hint 2".to_string()]),
            topic_tags: None,
        }
    }

    #[test]
    fn test_difficulty_level_from_i32() {
        assert_eq!(DifficultyLevel::try_from(1).unwrap(), DifficultyLevel::Easy);
        assert_eq!(
            DifficultyLevel::try_from(2).unwrap(),
            DifficultyLevel::Medium
        );
        assert_eq!(DifficultyLevel::try_from(3).unwrap(), DifficultyLevel::Hard);
        assert!(DifficultyLevel::try_from(0).is_err());
        assert!(DifficultyLevel::try_from(99).is_err());
        assert!(DifficultyLevel::try_from(-1).is_err());
    }

    #[test]
    fn test_problem_stat_question_title() {
        let problem = create_test_problem(1, "Two Sum", "two-sum", 1);
        assert_eq!(problem.stat.question_title(), "Two Sum");
    }

    #[test]
    fn test_problem_stat_question_title_slug() {
        let problem = create_test_problem(1, "Add Two Numbers", "add-two-numbers", 2);
        assert_eq!(problem.stat.question_title_slug(), "add-two-numbers");
    }

    #[test]
    fn test_problem_detail_clean_content() {
        let detail = create_test_problem_detail("1", "Test", "test");
        let cleaned = detail.clean_content();
        // The content should be processed (HTML converted)
        assert!(!cleaned.contains("<p>"));
        assert!(!cleaned.contains("</p>"));
    }

    #[test]
    fn test_problem_detail_example_testcases() {
        let detail = create_test_problem_detail("1", "Test", "test");
        let examples = detail.example_testcases.as_ref().unwrap();
        assert!(examples.contains("input1"));
        assert!(examples.contains("output1"));
        assert!(examples.contains("input2"));
        assert!(examples.contains("output2"));
    }

    #[test]
    fn test_problem_detail_get_rust_snippet() {
        let detail = create_test_problem_detail("1", "Test", "test");
        let snippet = detail.get_rust_snippet();
        assert!(snippet.is_some());
        assert!(snippet.unwrap().contains("impl Solution"));
    }

    #[test]
    fn test_acceptance_rate_calculation() {
        let problem = Problem {
            stat: Stat {
                question_id: 1,
                question__article__live: None,
                question__article__slug: None,
                question__title: Some("Test".to_string()),
                question__title_slug: "test".to_string(),
                question__hide: false,
                total_acs: 750,
                total_submitted: 1000,
                frontend_question_id: 1,
                is_new_question: false,
            },
            difficulty: Difficulty { level: 1 },
            paid_only: false,
            is_favor: false,
            frequency: 0,
            progress: 0,
            status: None,
        };

        let rate = problem.stat.total_acs as f64 / problem.stat.total_submitted as f64 * 100.0;
        assert_eq!(rate, 75.0);
    }

    #[test]
    fn test_difficulty_level_display() {
        let easy = DifficultyLevel::Easy;
        let medium = DifficultyLevel::Medium;
        let hard = DifficultyLevel::Hard;

        assert_eq!(easy as i32, 1);
        assert_eq!(medium as i32, 2);
        assert_eq!(hard as i32, 3);
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_show_execute_success() {
        use wiremock::{
            Mock, ResponseTemplate,
            matchers::{method, path},
        };

        let mock_server = wiremock::MockServer::start().await;
        let config = crate::config::Config::default();

        // Setup mock for problem list
        let problem_list = serde_json::json!({
            "user_name": "test_user",
            "num_solved": 1,
            "num_total": 1,
            "ac_easy": 1,
            "ac_medium": 0,
            "ac_hard": 0,
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
                    "status": null
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(problem_list))
            .mount(&mock_server)
            .await;

        // Setup mock for problem detail
        let graphql_response = serde_json::json!({
            "data": {
                "question": {
                    "questionId": "1",
                    "title": "Two Sum",
                    "titleSlug": "two-sum",
                    "content": "<p>Given an array of integers...</p>",
                    "difficulty": "Easy",
                    "exampleTestcases": "[2,7,11,15]\\n9",
                    "sampleTestCase": "[2,7,11,15]\\n9",
                    "metaData": null,
                    "codeSnippets": [
                        {
                            "lang": "Rust",
                            "langSlug": "rust",
                            "code": "impl Solution { pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> { } }"
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

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        let result = execute(&client, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_show_execute_problem_not_found() {
        use wiremock::{
            Mock, ResponseTemplate,
            matchers::{method, path},
        };

        let mock_server = wiremock::MockServer::start().await;
        let config = crate::config::Config::default();

        // Setup mock for problem list (empty)
        let problem_list = serde_json::json!({
            "user_name": "test_user",
            "num_solved": 0,
            "num_total": 0,
            "ac_easy": 0,
            "ac_medium": 0,
            "ac_hard": 0,
            "stat_status_pairs": []
        });

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(problem_list))
            .mount(&mock_server)
            .await;

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        // Should fail because problem 999 doesn't exist
        let result = execute(&client, 999).await;
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("problem not found"));
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_show_execute_different_difficulties() {
        use wiremock::{
            Mock, ResponseTemplate,
            matchers::{method, path},
        };

        let mock_server = wiremock::MockServer::start().await;
        let config = crate::config::Config::default();

        // Setup mock with problems of different difficulties
        let problem_list = serde_json::json!({
            "user_name": "test_user",
            "num_solved": 3,
            "num_total": 3,
            "ac_easy": 1,
            "ac_medium": 1,
            "ac_hard": 1,
            "stat_status_pairs": [
                {
                    "stat": {
                        "question_id": 1,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "Easy Problem",
                        "question__title_slug": "easy-problem",
                        "question__hide": false,
                        "total_acs": 1000,
                        "total_submitted": 2000,
                        "frontend_question_id": 1,
                        "is_new_question": false
                    },
                    "difficulty": {"level": 1},
                    "paid_only": false,
                    "is_favor": false,
                    "frequency": 0,
                    "progress": 0,
                    "status": null
                },
                {
                    "stat": {
                        "question_id": 2,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "Medium Problem",
                        "question__title_slug": "medium-problem",
                        "question__hide": false,
                        "total_acs": 500,
                        "total_submitted": 1000,
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
                        "total_acs": 100,
                        "total_submitted": 500,
                        "frontend_question_id": 3,
                        "is_new_question": false
                    },
                    "difficulty": {"level": 3},
                    "paid_only": false,
                    "is_favor": false,
                    "frequency": 0,
                    "progress": 0,
                    "status": null
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(problem_list))
            .mount(&mock_server)
            .await;

        // Setup GraphQL mock for all problems
        let graphql_response = serde_json::json!({
            "data": {
                "question": {
                    "questionId": "1",
                    "title": "Problem",
                    "titleSlug": "problem",
                    "content": "<p>Description</p>",
                    "difficulty": "Easy",
                    "exampleTestcases": "input\\noutput",
                    "sampleTestCase": "input\\noutput",
                    "metaData": null,
                    "codeSnippets": [],
                    "hints": [],
                    "topicTags": []
                }
            }
        });

        Mock::given(method("POST"))
            .and(path("/graphql"))
            .respond_with(ResponseTemplate::new(200).set_body_json(graphql_response))
            .mount(&mock_server)
            .await;

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        // Test showing problems of different difficulties
        for id in 1..=3 {
            let result = execute(&client, id).await;
            assert!(result.is_ok(), "Failed for problem {}", id);
        }
    }
}
