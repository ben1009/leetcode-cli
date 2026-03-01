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
