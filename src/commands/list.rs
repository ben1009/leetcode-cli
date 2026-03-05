//! List command - List all problems

use anyhow::Result;
use colored::Colorize;

use crate::{api::LeetCodeClient, problem::DifficultyLevel};

/// List all problems
pub async fn execute(
    client: &LeetCodeClient,
    difficulty: Option<String>,
    status: Option<String>,
) -> Result<()> {
    println!("{}", "Fetching problem list...".cyan());

    let problems = client.get_all_problems().await?;

    println!(
        "\n{:<6} {:<50} {:<10} {:<10}",
        "ID", "Title", "Difficulty", "Status"
    );
    println!("{}", "-".repeat(80));

    for problem in problems.iter() {
        let diff_str = match DifficultyLevel::try_from(problem.difficulty.level) {
            Ok(DifficultyLevel::Easy) => "Easy".green(),
            Ok(DifficultyLevel::Medium) => "Medium".yellow(),
            Ok(DifficultyLevel::Hard) => "Hard".red(),
            Err(_) => "Unknown".normal(),
        };

        let status_str = if problem.status == Some("ac".to_string()) {
            "✓ Solved".green()
        } else if problem.status == Some("notac".to_string()) {
            "~ Trying".yellow()
        } else {
            "○ New".normal()
        };

        if let Some(ref diff_filter) = difficulty
            && let Ok(level) = diff_filter.parse::<DifficultyLevel>()
            && problem.difficulty.level != level.level()
        {
            continue;
        }
        if let Some(ref status_filter) = status {
            let should_show = match status_filter.to_lowercase().as_str() {
                "solved" => problem.status == Some("ac".to_string()),
                "attempting" => problem.status == Some("notac".to_string()),
                "unsolved" => problem.status.is_none(),
                _ => true,
            };
            if !should_show {
                continue;
            }
        }

        println!(
            "{:<6} {:<50} {:<10} {:<10}",
            problem.stat.question_id,
            problem
                .stat
                .question_title()
                .chars()
                .take(48)
                .collect::<String>(),
            diff_str,
            status_str
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::problem::{Difficulty, Problem, Stat};

    fn create_test_problem(id: u32, title: &str, level: i32, status: Option<&str>) -> Problem {
        Problem {
            stat: Stat {
                question_id: id,
                question__article__live: None,
                question__article__slug: None,
                question__title: Some(title.to_string()),
                question__title_slug: title.to_lowercase().replace(' ', "-"),
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
            status: status.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_difficulty_level_try_from() {
        assert_eq!(DifficultyLevel::try_from(1).unwrap(), DifficultyLevel::Easy);
        assert_eq!(
            DifficultyLevel::try_from(2).unwrap(),
            DifficultyLevel::Medium
        );
        assert_eq!(DifficultyLevel::try_from(3).unwrap(), DifficultyLevel::Hard);
        assert!(DifficultyLevel::try_from(0).is_err());
        assert!(DifficultyLevel::try_from(4).is_err());
    }

    #[test]
    fn test_problem_status_display() {
        // Test solved status
        let problem_solved = create_test_problem(1, "Two Sum", 1, Some("ac"));
        assert_eq!(problem_solved.status, Some("ac".to_string()));

        // Test attempting status
        let problem_attempting = create_test_problem(2, "Add Two Numbers", 2, Some("notac"));
        assert_eq!(problem_attempting.status, Some("notac".to_string()));

        // Test unsolved status
        let problem_unsolved = create_test_problem(3, "Longest Substring", 3, None);
        assert!(problem_unsolved.status.is_none());
    }

    #[test]
    fn test_difficulty_filter_parsing() {
        // Test valid difficulty parsing
        assert!("easy".parse::<DifficultyLevel>().is_ok());
        assert!("Easy".parse::<DifficultyLevel>().is_ok());
        assert!("EASY".parse::<DifficultyLevel>().is_ok());
        assert!("medium".parse::<DifficultyLevel>().is_ok());
        assert!("hard".parse::<DifficultyLevel>().is_ok());

        // Test invalid difficulty
        assert!("invalid".parse::<DifficultyLevel>().is_err());
        assert!("".parse::<DifficultyLevel>().is_err());
    }

    #[test]
    fn test_status_filter_matching() {
        // Test status filter matching logic
        let test_cases = vec![
            ("solved", Some("ac".to_string()), true),
            ("solved", Some("notac".to_string()), false),
            ("solved", None, false),
            ("attempting", Some("notac".to_string()), true),
            ("attempting", Some("ac".to_string()), false),
            ("attempting", None, false),
            ("unsolved", None, true),
            ("unsolved", Some("ac".to_string()), false),
            ("unsolved", Some("notac".to_string()), false),
            ("invalid", Some("ac".to_string()), true), // unknown filter shows all
        ];

        for (filter, status, expected) in test_cases {
            let should_show = match filter {
                "solved" => status == Some("ac".to_string()),
                "attempting" => status == Some("notac".to_string()),
                "unsolved" => status.is_none(),
                _ => true,
            };
            assert_eq!(
                should_show, expected,
                "Failed for filter: {:?}, status: {:?}",
                filter, status
            );
        }
    }

    #[test]
    fn test_question_title_formatting() {
        let problem = create_test_problem(1, "Two Sum", 1, None);
        assert_eq!(problem.stat.question_title(), "Two Sum");
    }

    #[test]
    fn test_question_title_trimming() {
        let long_title = "a".repeat(100);
        let problem = create_test_problem(1, &long_title, 1, None);
        let title = problem.stat.question_title();
        let trimmed: String = title.chars().take(48).collect();
        assert!(trimmed.len() <= 48);
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_list_execute_with_mock_server() {
        use wiremock::{
            Mock, ResponseTemplate,
            matchers::{method, path},
        };

        let mock_server = wiremock::MockServer::start().await;
        let config = crate::config::Config::default();

        // Create test problem list
        let problem_list = serde_json::json!({
            "user_name": "test_user",
            "num_solved": 2,
            "num_total": 3,
            "ac_easy": 1,
            "ac_medium": 1,
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
                }
            ]
        });

        Mock::given(method("GET"))
            .and(path("/api/problems/all/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(problem_list))
            .mount(&mock_server)
            .await;

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        // Test execute without filters
        let result = execute(&client, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_list_execute_with_difficulty_filter() {
        use wiremock::{
            Mock, ResponseTemplate,
            matchers::{method, path},
        };

        let mock_server = wiremock::MockServer::start().await;
        let config = crate::config::Config::default();

        let problem_list = serde_json::json!({
            "user_name": "test_user",
            "num_solved": 0,
            "num_total": 2,
            "ac_easy": 0,
            "ac_medium": 0,
            "ac_hard": 0,
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
                        "question__title": "Hard Problem",
                        "question__title_slug": "hard-problem",
                        "question__hide": false,
                        "total_acs": 500,
                        "total_submitted": 1000,
                        "frontend_question_id": 2,
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

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        // Test with difficulty filter
        let result = execute(&client, Some("easy".to_string()), None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_list_execute_with_status_filter() {
        use wiremock::{
            Mock, ResponseTemplate,
            matchers::{method, path},
        };

        let mock_server = wiremock::MockServer::start().await;
        let config = crate::config::Config::default();

        let problem_list = serde_json::json!({
            "user_name": "test_user",
            "num_solved": 1,
            "num_total": 3,
            "ac_easy": 1,
            "ac_medium": 0,
            "ac_hard": 0,
            "stat_status_pairs": [
                {
                    "stat": {
                        "question_id": 1,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "Solved Problem",
                        "question__title_slug": "solved-problem",
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
                    "status": "ac"
                },
                {
                    "stat": {
                        "question_id": 2,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "Attempting Problem",
                        "question__title_slug": "attempting-problem",
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
                    "status": "notac"
                },
                {
                    "stat": {
                        "question_id": 3,
                        "question__article__live": null,
                        "question__article__slug": null,
                        "question__title": "New Problem",
                        "question__title_slug": "new-problem",
                        "question__hide": false,
                        "total_acs": 100,
                        "total_submitted": 200,
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

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        // Test with different status filters
        let result_solved = execute(&client, None, Some("solved".to_string())).await;
        assert!(result_solved.is_ok());

        let result_attempting = execute(&client, None, Some("attempting".to_string())).await;
        assert!(result_attempting.is_ok());

        let result_unsolved = execute(&client, None, Some("unsolved".to_string())).await;
        assert!(result_unsolved.is_ok());
    }
}
