//! Command modules for leetcode-cli
//!
//! Each submodule handles a specific CLI subcommand.

pub mod download;
pub mod list;
pub mod login;
pub mod pick;
pub mod show;
pub mod submit;
pub mod test;

use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::{
    api::SubmissionResult,
    problem::{DifficultyLevel, Problem},
};

/// Prompt the user for input with a message
pub fn prompt_input(message: &str) -> Result<String> {
    println!("{}", message.cyan());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

/// Prompt the user for a yes/no confirmation
/// Returns true if the user confirms (Y/n), false if not (n)
pub fn prompt_confirm(message: &str) -> Result<bool> {
    println!("{}", message.yellow());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_lowercase() != "n")
}

/// Print a summary of a problem
pub fn print_problem_summary(problem: &Problem) {
    println!("\n{}", "═".repeat(80).cyan());
    println!(
        "{} {}. {}",
        "✓ Found Problem".bold().green(),
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
    println!(
        "{} {}/{}",
        "Solved By:".bold(),
        problem.stat.total_acs,
        problem.stat.total_submitted
    );
    println!(
        "{} https://leetcode.com/problems/{}",
        "Link:".bold(),
        problem.stat.question_title_slug()
    );
}

/// Print the result of a submission
pub fn print_submission_result(result: &SubmissionResult) {
    match result.status_code {
        10 => {
            println!("{}", "✓ Accepted!".green().bold());
            println!(
                "  Runtime: {} ms (faster than {:.1}%)",
                result.status_runtime, result.runtime_percentile
            );
            println!(
                "  Memory: {} MB (less than {:.1}%)",
                result.status_memory, result.memory_percentile
            );
        }
        11 => {
            println!("{}", "✗ Wrong Answer".red().bold());
            println!("  {}", result.status_msg);
            if let Some(ref output) = result.code_output {
                println!("  Your output: {}", output);
            }
            if let Some(ref expected) = result.expected_output {
                println!("  Expected: {}", expected);
            }
        }
        14 => {
            println!("{}", "✗ Time Limit Exceeded".red().bold());
        }
        15 => {
            println!("{}", "✗ Runtime Error".red().bold());
            if let Some(ref error) = result.full_runtime_error {
                println!("  {}", error);
            }
        }
        20 => {
            println!("{}", "✗ Compile Error".red().bold());
            if let Some(ref error) = result.full_compile_error {
                println!("  {}", error);
            }
        }
        _ => {
            println!("{} {}", "Status:".bold(), result.status_msg);
        }
    }
}

/// Find the solution file for a problem
pub fn find_solution_file(id: u32, file: Option<PathBuf>) -> Result<PathBuf> {
    if let Some(f) = file {
        return Ok(f);
    }

    // Try to find the solution file automatically
    // First, try new structure: src/lib.rs
    let entries: Vec<_> = std::fs::read_dir(".")?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with(&format!("{:04}_", id))
        })
        .collect();

    if entries.is_empty() {
        anyhow::bail!("Problem directory not found. Please specify with --file");
    }

    let problem_dir = entries[0].path();

    // Try new structure first: src/lib.rs
    let lib_rs = problem_dir.join("src/lib.rs");
    if lib_rs.exists() {
        Ok(lib_rs)
    } else {
        // Try legacy structure: solution.rs
        let solution_rs = problem_dir.join("solution.rs");
        if solution_rs.exists() {
            Ok(solution_rs)
        } else {
            anyhow::bail!("Solution file not found. Expected either src/lib.rs or solution.rs");
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_find_solution_file_with_explicit_path() {
        let temp_dir = TempDir::new().unwrap();
        let solution_file = temp_dir.path().join("solution.rs");
        std::fs::write(&solution_file, "fn main() {}").unwrap();

        let result = find_solution_file(1, Some(solution_file.clone()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), solution_file);
    }

    #[test]
    fn test_find_solution_file_not_found() {
        // Create a temp directory that won't have the problem directory
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let result = find_solution_file(999, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_find_solution_file_cargo_structure() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Create problem directory with Cargo structure
        let problem_dir = temp_dir.path().join("0001_two_sum");
        let src_dir = problem_dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        let lib_rs = src_dir.join("lib.rs");
        std::fs::write(&lib_rs, "pub struct Solution;").unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let result = find_solution_file(1, None);
        assert!(result.is_ok());
        // Compare file names since paths may be canonicalized differently
        let found_path = result.unwrap();
        assert!(found_path.to_string_lossy().contains("0001_two_sum"));
        assert!(found_path.to_string_lossy().contains("lib.rs"));

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_find_solution_file_legacy_structure() {
        let temp_dir = TempDir::new().unwrap();
        let original_dir = std::env::current_dir().unwrap();

        // Create problem directory with legacy structure
        let problem_dir = temp_dir.path().join("0002_add_two_numbers");
        std::fs::create_dir(&problem_dir).unwrap();
        let solution_rs = problem_dir.join("solution.rs");
        std::fs::write(&solution_rs, "pub struct Solution;").unwrap();

        std::env::set_current_dir(&temp_dir).unwrap();

        let result = find_solution_file(2, None);
        assert!(result.is_ok());
        // Compare file names since paths may be canonicalized differently
        let found_path = result.unwrap();
        assert!(
            found_path
                .to_string_lossy()
                .contains("0002_add_two_numbers")
        );
        assert!(found_path.to_string_lossy().contains("solution.rs"));

        std::env::set_current_dir(original_dir).unwrap();
    }

    #[test]
    fn test_print_problem_summary() {
        use crate::problem::{Difficulty, Stat};

        let problem = Problem {
            stat: Stat {
                question_id: 1,
                question__article__live: None,
                question__article__slug: None,
                question__title: Some("Two Sum".to_string()),
                question__title_slug: "two-sum".to_string(),
                question__hide: false,
                total_acs: 1000000,
                total_submitted: 2000000,
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

        // Just make sure it doesn't panic
        print_problem_summary(&problem);
    }

    #[test]
    fn test_print_submission_result_accepted() {
        let result = SubmissionResult {
            status_code: 10,
            status_msg: "Accepted".to_string(),
            status_runtime: "0 ms".to_string(),
            status_memory: "2.1 MB".to_string(),
            runtime_percentile: 95.5,
            memory_percentile: 80.0,
            code_output: None,
            expected_output: None,
            full_runtime_error: None,
            full_compile_error: None,
            total_correct: Some(100),
            total_testcases: Some(100),
            input_formatted: None,
        };

        // Just make sure it doesn't panic
        print_submission_result(&result);
    }

    #[test]
    fn test_print_submission_result_wrong_answer() {
        let result = SubmissionResult {
            status_code: 11,
            status_msg: "Wrong Answer".to_string(),
            status_runtime: "0 ms".to_string(),
            status_memory: "2.1 MB".to_string(),
            runtime_percentile: 0.0,
            memory_percentile: 0.0,
            code_output: Some("[1, 2]".to_string()),
            expected_output: Some("[0, 1]".to_string()),
            full_runtime_error: None,
            full_compile_error: None,
            total_correct: Some(50),
            total_testcases: Some(100),
            input_formatted: None,
        };

        // Just make sure it doesn't panic
        print_submission_result(&result);
    }

    #[test]
    fn test_print_submission_result_compile_error() {
        let result = SubmissionResult {
            status_code: 20,
            status_msg: "Compile Error".to_string(),
            status_runtime: "0 ms".to_string(),
            status_memory: "0 MB".to_string(),
            runtime_percentile: 0.0,
            memory_percentile: 0.0,
            code_output: None,
            expected_output: None,
            full_runtime_error: None,
            full_compile_error: Some("error: expected semicolon".to_string()),
            total_correct: None,
            total_testcases: None,
            input_formatted: None,
        };

        // Just make sure it doesn't panic
        print_submission_result(&result);
    }

    #[test]
    fn test_print_submission_result_unknown_status() {
        let result = SubmissionResult {
            status_code: 999,
            status_msg: "Unknown Status".to_string(),
            status_runtime: "0 ms".to_string(),
            status_memory: "0 MB".to_string(),
            runtime_percentile: 0.0,
            memory_percentile: 0.0,
            code_output: None,
            expected_output: None,
            full_runtime_error: None,
            full_compile_error: None,
            total_correct: None,
            total_testcases: None,
            input_formatted: None,
        };

        // Just make sure it doesn't panic
        print_submission_result(&result);
    }
}
