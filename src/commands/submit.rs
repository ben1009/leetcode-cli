//! Submit command - Submit solution to LeetCode

use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::{
    api::LeetCodeClient,
    commands::{find_solution_file, print_submission_result},
};

/// Submit solution to LeetCode
pub async fn execute(client: &LeetCodeClient, id: u32, file: Option<PathBuf>) -> Result<()> {
    let solution_file = find_solution_file(id, file)?;

    println!(
        "{}",
        format!("Submitting solution for problem {id}...").cyan()
    );
    let result = client.submit(id, &solution_file).await?;
    print_submission_result(&result);

    Ok(())
}
