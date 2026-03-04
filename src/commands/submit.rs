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

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use crate::commands::TestDirGuard;

    #[test]
    #[serial_test::serial]
    fn test_find_solution_file_for_submit() {
        let temp_dir = TempDir::new().unwrap();

        // Create problems directory with a solution file
        fs::create_dir_all(temp_dir.path().join("src/problems")).unwrap();
        fs::write(
            temp_dir.path().join("src/problems/p0001_two_sum.rs"),
            "pub struct Solution;",
        )
        .unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        // Test finding solution file
        let result = super::find_solution_file(1, None);
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains("p0001_two_sum.rs"));
    }

    #[test]
    #[serial_test::serial]
    fn test_find_solution_file_not_found() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir_all(temp_dir.path().join("src/problems")).unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        // Test finding non-existent problem
        let result = super::find_solution_file(999, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }

    #[test]
    #[serial_test::serial]
    fn test_find_solution_file_with_explicit_path() {
        let temp_dir = TempDir::new().unwrap();
        let custom_file = temp_dir.path().join("custom_solution.rs");
        fs::write(&custom_file, "pub struct Solution;").unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        // Test with explicit file path
        let result = super::find_solution_file(1, Some(custom_file.clone()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), custom_file);
    }
}
