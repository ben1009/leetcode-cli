//! Download command - Download problem to local workspace

use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::{api::LeetCodeClient, template::CodeTemplate};

/// Sanitize a string to be safe for use in a file/directory name.
/// Removes path separators and other potentially dangerous characters.
fn sanitize_file_name(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|'))
        .collect()
}

/// Add a module declaration to src/problems/mod.rs if it doesn't exist
fn add_module_declaration(module_name: &str) -> Result<()> {
    let mod_path = PathBuf::from("src/problems/mod.rs");

    // Create problems directory if it doesn't exist
    std::fs::create_dir_all("src/problems")?;

    let mod_decl = format!("pub mod {module_name};");

    // Read existing content or create default
    let content = if mod_path.exists() {
        std::fs::read_to_string(&mod_path)?
    } else {
        "//! LeetCode problem solutions\n//!\n//! Each module contains the solution for a specific LeetCode problem.\n\n".to_string()
    };

    // Check if module already declared
    if content.contains(&mod_decl) {
        return Ok(());
    }

    // Append module declaration
    let updated = format!("{content}{mod_decl}\n");
    std::fs::write(&mod_path, updated)?;

    Ok(())
}

/// Download problem to local workspace
pub async fn execute(client: &LeetCodeClient, id: u32, _output: PathBuf) -> Result<()> {
    println!("{}", format!("Downloading problem {id}...").cyan());

    let problem = client
        .get_problem_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("problem not found: ID {id}"))?;

    let detail = client
        .get_problem_detail(&problem.stat.question_title_slug())
        .await?;

    // Create module name: p0001_two_sum (prefix with 'p' for valid Rust identifier)
    let slug = sanitize_file_name(&problem.stat.question_title_slug());
    let module_name = format!("p{:04}_{}", id, slug.replace("-", "_"));
    let file_name = format!("{module_name}.rs");

    // Ensure problems directory exists
    let problems_dir = PathBuf::from("src/problems");
    std::fs::create_dir_all(&problems_dir)?;

    // Generate code template
    let template = CodeTemplate::new(&detail);
    let code_file = problems_dir.join(&file_name);
    template.write_rust_template(&code_file)?;

    // Add module declaration
    add_module_declaration(&module_name)?;

    // Write test cases to separate directory
    let test_cases_dir = problems_dir.join("test_cases");
    std::fs::create_dir_all(&test_cases_dir)?;
    let test_file = test_cases_dir.join(format!("{module_name}.json"));
    template.write_test_cases(&test_file)?;

    println!(
        "{}",
        format!("✓ Problem downloaded: {}", code_file.display()).green()
    );
    println!("  - Solution: {}", code_file.display());
    println!("  - Test cases: {}", test_file.display());
    println!();
    println!("{}", "To run tests:".cyan());
    println!("  cargo test {module_name}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::commands::TestDirGuard;

    #[test]
    fn test_sanitize_file_name_normal() {
        assert_eq!(sanitize_file_name("two-sum"), "two-sum");
        assert_eq!(sanitize_file_name("add-two-numbers"), "add-two-numbers");
    }

    #[test]
    fn test_sanitize_file_name_removes_path_traversal() {
        assert_eq!(sanitize_file_name("../../../etc/passwd"), "......etcpasswd");
        assert_eq!(sanitize_file_name("..\\\\..\\\\windows"), "....windows");
    }

    #[test]
    fn test_sanitize_file_name_removes_invalid_chars() {
        assert_eq!(sanitize_file_name("test:name"), "testname");
        assert_eq!(sanitize_file_name("test*name"), "testname");
        assert_eq!(sanitize_file_name("test?name"), "testname");
        assert_eq!(sanitize_file_name("test\"name"), "testname");
        assert_eq!(sanitize_file_name("test<name>"), "testname");
        assert_eq!(sanitize_file_name("test|name"), "testname");
    }

    #[test]
    fn test_sanitize_file_name_empty() {
        assert_eq!(sanitize_file_name(""), "");
    }

    #[test]
    fn test_sanitize_file_name_all_invalid() {
        assert_eq!(sanitize_file_name("/\\:*?\"<>|"), "");
    }

    #[test]
    #[serial_test::serial]
    fn test_add_module_declaration_creates_new_file() {
        let temp_dir = TempDir::new().unwrap();

        // Create src directory
        fs::create_dir_all(temp_dir.path().join("src/problems")).unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let result = add_module_declaration("p0001_two_sum");
        assert!(result.is_ok());

        let content = fs::read_to_string("src/problems/mod.rs").unwrap();
        assert!(content.contains("pub mod p0001_two_sum;"));
        assert!(content.contains("//! LeetCode problem solutions"));
    }

    #[test]
    #[serial_test::serial]
    fn test_add_module_declaration_appends_to_existing() {
        let temp_dir = TempDir::new().unwrap();

        // Create existing mod.rs
        fs::create_dir_all(temp_dir.path().join("src/problems")).unwrap();
        fs::write(
            temp_dir.path().join("src/problems/mod.rs"),
            "pub mod p0001_two_sum;\n",
        )
        .unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let result = add_module_declaration("p0002_add_two_numbers");
        assert!(result.is_ok());

        let content = fs::read_to_string("src/problems/mod.rs").unwrap();
        assert!(content.contains("pub mod p0001_two_sum;"));
        assert!(content.contains("pub mod p0002_add_two_numbers;"));
    }

    #[test]
    #[serial_test::serial]
    fn test_add_module_declaration_skips_duplicate() {
        let temp_dir = TempDir::new().unwrap();

        // Create existing mod.rs with the module already declared
        fs::create_dir_all(temp_dir.path().join("src/problems")).unwrap();
        fs::write(
            temp_dir.path().join("src/problems/mod.rs"),
            "pub mod p0001_two_sum;\n",
        )
        .unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let result = add_module_declaration("p0001_two_sum");
        assert!(result.is_ok());

        let content = fs::read_to_string("src/problems/mod.rs").unwrap();
        // Should only appear once
        let count = content.matches("pub mod p0001_two_sum;").count();
        assert_eq!(count, 1);
    }

    /// Create a test problem list for mocking
    fn create_test_problem_list() -> serde_json::Value {
        serde_json::json!({
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
                    "status": "ac"
                }
            ]
        })
    }

    async fn setup_mock_server() -> (wiremock::MockServer, crate::config::Config) {
        let mock_server = wiremock::MockServer::start().await;
        let config = crate::config::Config::default();
        (mock_server, config)
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_download_execute_success() {
        let (mock_server, config) = setup_mock_server().await;
        let temp_dir = TempDir::new().unwrap();

        // Setup mock for problem list
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/problems/all/"))
            .respond_with(
                wiremock::ResponseTemplate::new(200).set_body_json(create_test_problem_list()),
            )
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
                    "exampleTestcases": "[2,7,11,15]\n9",
                    "sampleTestCase": "[2,7,11,15]\n9",
                    "metaData": null,
                    "codeSnippets": [
                        {
                            "lang": "Rust",
                            "langSlug": "rust",
                            "code": "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        \n    }\n}"
                        }
                    ],
                    "hints": ["Use a hash map"],
                    "topicTags": [{"name": "Array", "slug": "array"}]
                }
            }
        });

        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/graphql"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(graphql_response))
            .mount(&mock_server)
            .await;

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        // Create src/problems directory
        fs::create_dir_all("src/problems").unwrap();

        // Execute download
        let result = execute(&client, 1, PathBuf::from(".")).await;
        assert!(result.is_ok());

        // Verify files were created
        assert!(fs::metadata("src/problems/p0001_two_sum.rs").is_ok());
        assert!(fs::metadata("src/problems/test_cases/p0001_two_sum.json").is_ok());

        // Verify mod.rs was updated
        let mod_content = fs::read_to_string("src/problems/mod.rs").unwrap();
        assert!(mod_content.contains("pub mod p0001_two_sum;"));
    }

    #[tokio::test]
    #[serial_test::serial]
    #[cfg_attr(miri, ignore = "Miri doesn't support TCP sockets")]
    async fn test_download_execute_problem_not_found() {
        let (mock_server, config) = setup_mock_server().await;
        let temp_dir = TempDir::new().unwrap();

        // Setup mock for problem list (only has problem 1)
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/api/problems/all/"))
            .respond_with(
                wiremock::ResponseTemplate::new(200).set_body_json(create_test_problem_list()),
            )
            .mount(&mock_server)
            .await;

        let client = crate::api::LeetCodeClient::new_with_base_url(config, mock_server.uri())
            .await
            .unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        // Try to download non-existent problem
        let result = execute(&client, 999, PathBuf::from(".")).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("problem not found")
        );
    }
}
