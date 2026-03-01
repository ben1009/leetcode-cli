//! Download command - Download problem to local workspace

use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::{api::LeetCodeClient, template::CodeTemplate};

/// Download problem to local workspace
pub async fn execute(client: &LeetCodeClient, id: u32, output: PathBuf) -> Result<()> {
    println!("{}", format!("Downloading problem {id}...").cyan());

    let problem = client
        .get_problem_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Problem not found"))?;

    let detail = client
        .get_problem_detail(&problem.stat.question_title_slug())
        .await?;

    // Create problem directory
    let problem_dir = output.join(format!(
        "{:04}_{}",
        id,
        problem.stat.question_title_slug().replace("-", "_")
    ));
    std::fs::create_dir_all(&problem_dir)?;

    // Create src directory
    let src_dir = problem_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;

    // Generate code template
    let template = CodeTemplate::new(&detail);
    let code_file = src_dir.join("lib.rs");
    template.write_rust_template(&code_file)?;

    // Write Cargo.toml
    let cargo_file = problem_dir.join("Cargo.toml");
    template.write_cargo_toml(&cargo_file)?;

    // Write problem description
    let desc_file = problem_dir.join("README.md");
    template.write_description(&desc_file)?;

    // Write test cases
    let test_file = problem_dir.join("test_cases.json");
    template.write_test_cases(&test_file)?;

    println!(
        "{}",
        format!("✓ Problem downloaded to: {}", problem_dir.display()).green()
    );
    println!("  - Solution: {}", code_file.display());
    println!("  - Cargo.toml: {}", cargo_file.display());
    println!("  - Description: {}", desc_file.display());
    println!("  - Test cases: {}", test_file.display());
    println!();
    println!("{}", "To run tests:".cyan());
    println!("  cd {}", problem_dir.display());
    println!("  cargo test");

    Ok(())
}
