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

        if let Some(ref diff_filter) = difficulty {
            if let Some(level) = DifficultyLevel::from_str(diff_filter) {
                if problem.difficulty.level != level.level() {
                    continue;
                }
            }
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
