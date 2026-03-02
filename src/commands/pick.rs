//! Pick command - Select a random problem or specific problem by ID

use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::{
    api::LeetCodeClient,
    commands::{download, print_problem_summary, prompt_confirm},
};

/// Pick a random problem or specific problem by ID
pub async fn execute(
    client: &LeetCodeClient,
    id: Option<u32>,
    difficulty: Option<String>,
    tag: Option<String>,
) -> Result<()> {
    println!("{}", "Fetching problems...".cyan());

    let problem = if let Some(problem_id) = id {
        client.get_problem_by_id(problem_id).await?
    } else {
        client
            .get_random_problem(difficulty.as_deref(), tag.as_deref())
            .await?
    };

    if let Some(p) = problem {
        print_problem_summary(&p);

        // Ask if user wants to download
        if prompt_confirm("\nDownload this problem? [Y/n]")? {
            download::execute(client, p.stat.question_id, PathBuf::from(".")).await?;
        }
    } else {
        println!("{}", "No problem found matching the criteria.".red());
    }

    Ok(())
}
