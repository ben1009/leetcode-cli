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
