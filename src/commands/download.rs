//! Download command - Download problem to local workspace

use std::path::PathBuf;

use anyhow::Result;
use colored::Colorize;

use crate::{api::LeetCodeClient, template::CodeTemplate};

/// Update workspace members in root Cargo.toml to include new problem patterns if needed.
/// This ensures rust-analyzer can discover newly downloaded problems.
fn update_workspace_if_needed(id: u32) -> Result<()> {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")?;

    // Determine which pattern this problem ID belongs to
    let pattern = match id {
        0..=999 => "0*",
        1000..=1999 => "1*",
        2000..=2999 => "2*",
        3000..=3999 => "3*",
        4000..=4999 => "4*",
        _ => return Ok(()), // Don't modify for very high IDs
    };

    // Check if pattern is already in members
    if cargo_toml.contains(&format!("\"{pattern}\"")) {
        return Ok(());
    }

    // Add the pattern to workspace members
    let updated = cargo_toml.replace(
        "members = [\".\", \"0*\"]",
        &format!("members = [\".\", \"0*\", \"{pattern}\"]")
    );

    std::fs::write("Cargo.toml", updated)?;
    println!("{}", format!("Updated workspace to include pattern '{}'", pattern).yellow());

    Ok(())
}

/// Sanitize a string to be safe for use in a directory name.
/// Removes path separators and other potentially dangerous characters.
fn sanitize_dir_name(name: &str) -> String {
    name.chars()
        .filter(|c| !matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|'))
        .collect()
}

/// Download problem to local workspace
pub async fn execute(client: &LeetCodeClient, id: u32, output: PathBuf) -> Result<()> {
    println!("{}", format!("Downloading problem {id}...").cyan());

    let problem = client
        .get_problem_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("problem not found: ID {id}"))?;

    let detail = client
        .get_problem_detail(&problem.stat.question_title_slug())
        .await?;

    // Create problem directory (sanitize slug to prevent path traversal)
    let slug = sanitize_dir_name(&problem.stat.question_title_slug());
    let problem_dir = output.join(format!("{:04}_{}", id, slug.replace("-", "_")));
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

    // Write test cases
    let test_file = problem_dir.join("test_cases.json");
    template.write_test_cases(&test_file)?;

    // Update workspace to include this problem pattern (for rust-analyzer)
    update_workspace_if_needed(id)?;

    println!(
        "{}",
        format!("✓ Problem downloaded to: {}", problem_dir.display()).green()
    );
    println!("  - Solution (with description): {}", code_file.display());
    println!("  - Cargo.toml: {}", cargo_file.display());
    println!("  - Test cases: {}", test_file.display());
    println!();
    println!("{}", "To run tests:".cyan());
    println!("  cd {}", problem_dir.display());
    println!("  cargo test");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_dir_name_normal() {
        assert_eq!(sanitize_dir_name("two-sum"), "two-sum");
        assert_eq!(sanitize_dir_name("add-two-numbers"), "add-two-numbers");
    }

    #[test]
    fn test_sanitize_dir_name_removes_path_traversal() {
        assert_eq!(sanitize_dir_name("../../../etc/passwd"), "......etcpasswd");
        assert_eq!(sanitize_dir_name("..\\\\..\\\\windows"), "....windows");
    }

    #[test]
    fn test_sanitize_dir_name_removes_invalid_chars() {
        assert_eq!(sanitize_dir_name("test:name"), "testname");
        assert_eq!(sanitize_dir_name("test*name"), "testname");
        assert_eq!(sanitize_dir_name("test?name"), "testname");
        assert_eq!(sanitize_dir_name("test\"name"), "testname");
        assert_eq!(sanitize_dir_name("test<name>"), "testname");
        assert_eq!(sanitize_dir_name("test|name"), "testname");
    }

    #[test]
    fn test_sanitize_dir_name_empty() {
        assert_eq!(sanitize_dir_name(""), "");
    }

    #[test]
    fn test_sanitize_dir_name_all_invalid() {
        assert_eq!(sanitize_dir_name("/\\:*?\"<>|"), "");
    }
}
