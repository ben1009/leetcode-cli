use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Result, anyhow};
use colored::*;

use crate::commands::find_problem_directory;

pub struct TestRunner {
    problem_id: u32,
    #[allow(dead_code)]
    test_file: Option<PathBuf>,
    problem_dir: PathBuf,
}

impl TestRunner {
    pub fn new(problem_id: u32, test_file: Option<PathBuf>) -> Result<Self> {
        // Find problem directory using the shared helper
        let problem_dir = find_problem_directory(problem_id)?;

        Ok(Self {
            problem_id,
            test_file,
            problem_dir,
        })
    }

    pub async fn run(&self) -> Result<()> {
        println!(
            "{}",
            format!("Running tests for problem {}...", self.problem_id).cyan()
        );

        // Check if this is a Cargo project (new structure)
        let cargo_toml = self.problem_dir.join("Cargo.toml");
        let lib_rs = self.problem_dir.join("src/lib.rs");
        let solution_rs = self.problem_dir.join("solution.rs");

        if cargo_toml.exists() && lib_rs.exists() {
            // New structure: run cargo test directly in the project directory
            self.run_cargo_test_in_dir(&self.problem_dir).await
        } else if solution_rs.exists() {
            // Old structure: create temp project
            self.run_cargo_test_legacy(&solution_rs).await
        } else {
            Err(anyhow!(
                "Solution file not found. Expected either:\n  - {}/src/lib.rs (new format)\n  - {}/solution.rs (old format)",
                self.problem_dir.display(),
                self.problem_dir.display()
            ))
        }
    }

    async fn run_cargo_test_in_dir(&self, project_dir: &Path) -> Result<()> {
        println!("{}", "Running cargo test...".cyan());

        let output = Command::new("cargo")
            .arg("test")
            .current_dir(project_dir)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Print output with formatting
        if !stdout.is_empty() {
            println!("\n{}", "Test Output:".bold());
            self.format_test_output(&stdout);
        }

        if !stderr.is_empty()
            && !stderr.contains("Compiling")
            && !stderr.contains("Finished")
            && !stderr.contains("Running")
        {
            println!("\n{}", "Compiler Messages:".yellow());
            println!("{}", stderr);
        }

        // Check test results
        if output.status.success() {
            println!("\n{}", "✓ All tests passed!".green().bold());
        } else {
            println!("\n{}", "✗ Some tests failed".red().bold());
        }

        Ok(())
    }

    async fn run_cargo_test_legacy(&self, solution_file: &Path) -> Result<()> {
        // Create a temporary Cargo project for testing (legacy support)
        let temp_dir = std::env::temp_dir().join(format!("leetcode_test_{}", self.problem_id));

        // Clean up old temp directory if exists
        if temp_dir.exists() {
            std::fs::remove_dir_all(&temp_dir)?;
        }

        // Create temporary project structure
        std::fs::create_dir_all(temp_dir.join("src"))?;

        // Create Cargo.toml
        let cargo_toml = r#"
[package]
name = "temp_solution"
version = "0.1.0"
edition = "2021"

[dependencies]
"#;
        std::fs::write(temp_dir.join("Cargo.toml"), cargo_toml)?;

        // Copy solution file
        let solution_content = std::fs::read_to_string(solution_file)?;
        std::fs::write(temp_dir.join("src/lib.rs"), &solution_content)?;

        // Run tests
        println!("{}", "Compiling and running tests...".cyan());

        let output = Command::new("cargo")
            .arg("test")
            .current_dir(&temp_dir)
            .output()?;

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Print output with formatting
        if !stdout.is_empty() {
            println!("\n{}", "Test Output:".bold());
            self.format_test_output(&stdout);
        }

        if !stderr.is_empty()
            && !stderr.contains("Compiling")
            && !stderr.contains("Finished")
            && !stderr.contains("Running")
        {
            println!("\n{}", "Compiler Messages:".yellow());
            println!("{}", stderr);
        }

        // Check test results
        if output.status.success() {
            println!("\n{}", "✓ All tests passed!".green().bold());
        } else {
            println!("\n{}", "✗ Some tests failed".red().bold());
        }

        // Clean up
        std::fs::remove_dir_all(&temp_dir)?;

        Ok(())
    }

    fn format_test_output(&self, output: &str) {
        for line in output.lines() {
            if line.contains("test result: ok") {
                println!("{}", line.green());
            } else if line.contains("test result: FAILED") {
                println!("{}", line.red());
            } else if line.contains("test ... ok") {
                println!("  {}", line.green());
            } else if line.contains("test ... FAILED") {
                println!("  {}", line.red());
            } else if line.contains("running") {
                println!("{}", line.cyan());
            } else {
                println!("{}", line);
            }
        }
    }

    #[allow(dead_code)]
    pub fn run_custom_tests(&self, test_file: &Path) -> Result<()> {
        println!(
            "{}",
            format!("Running custom tests from {}...", test_file.display()).cyan()
        );

        // Load custom test cases
        let test_content = std::fs::read_to_string(test_file)?;
        let test_cases: serde_json::Value = serde_json::from_str(&test_content)?;

        println!("\n{}", "Custom Test Cases:".bold());
        println!("{}", "-".repeat(60));

        if let Some(cases) = test_cases.get("test_cases").and_then(|t| t.as_array()) {
            for (i, case) in cases.iter().enumerate() {
                println!("\n{} {}", "Test Case".bold(), format!("#{}", i + 1).cyan());

                if let Some(input) = case.get("input") {
                    println!("  {} {}", "Input:".bold(), input);
                }

                if let Some(expected) = case.get("expected") {
                    println!("  {} {}", "Expected:".bold(), expected);
                }

                if let Some(explanation) = case.get("explanation") {
                    println!("  {} {}", "Explanation:".italic(), explanation);
                }
            }
        }

        println!("\n{}", "-".repeat(60));
        println!(
            "{}",
            "Run 'cargo test' in the problem directory to execute tests.".yellow()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;
    use crate::commands::{TestDirGuard, find_problem_directory as find_problem_dir};

    #[test]
    #[serial_test::serial]
    fn test_test_runner_creation() {
        let temp_dir = TempDir::new().unwrap();
        let problem_dir = temp_dir.path().join("0001_two_sum");
        fs::create_dir(&problem_dir).unwrap();
        fs::write(problem_dir.join("solution.rs"), "fn main() {}").unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let runner = TestRunner::new(1, None);
        assert!(runner.is_ok());
    }

    #[test]
    #[serial_test::serial]
    fn test_find_problem_directory_with_prefix() {
        let temp_dir = TempDir::new().unwrap();
        let problem_dir = temp_dir.path().join("0001_two_sum");
        fs::create_dir(&problem_dir).unwrap();
        fs::write(problem_dir.join("solution.rs"), "fn main() {}").unwrap();

        // Store the expected path before moving temp_dir into the guard
        let expected_canonical = problem_dir.canonicalize().unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let found = find_problem_dir(1);
        assert!(found.is_ok());
        // Compare canonicalized paths to handle macOS /var vs /private/var symlink
        let found_canonical = found.unwrap().canonicalize().unwrap();
        assert_eq!(found_canonical, expected_canonical);
    }

    #[test]
    #[serial_test::serial]
    fn test_find_problem_directory_cargo_structure() {
        let temp_dir = TempDir::new().unwrap();
        let expected_path = temp_dir.path().canonicalize().unwrap();
        let src_dir = temp_dir.path().join("src");
        fs::create_dir(&src_dir).unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();
        fs::write(src_dir.join("lib.rs"), "pub fn test() {}").unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let found = find_problem_dir(999);
        assert!(found.is_ok());
        // Compare canonicalized paths to handle macOS /var vs /private/var symlink
        let found_canonical = found.unwrap().canonicalize().unwrap();
        assert_eq!(found_canonical, expected_path);
    }

    #[test]
    #[serial_test::serial]
    fn test_find_problem_directory_not_found() {
        let temp_dir = TempDir::new().unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let found = find_problem_dir(999);
        assert!(found.is_err());
    }

    #[test]
    #[serial_test::serial]
    fn test_format_test_output_ok() {
        let temp_dir = TempDir::new().unwrap();
        let problem_dir = temp_dir.path().join("0001_test");
        fs::create_dir(&problem_dir).unwrap();
        fs::write(problem_dir.join("solution.rs"), "").unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let runner = TestRunner::new(1, None).unwrap();

        // This test mainly ensures format_test_output doesn't panic
        let output = "running 3 tests\ntest tests::test_one ... ok\ntest tests::test_two ... ok\ntest result: ok. 3 passed; 0 failed";
        runner.format_test_output(output);
    }

    #[test]
    #[serial_test::serial]
    fn test_format_test_output_failed() {
        let temp_dir = TempDir::new().unwrap();
        let problem_dir = temp_dir.path().join("0001_test");
        fs::create_dir(&problem_dir).unwrap();
        fs::write(problem_dir.join("solution.rs"), "").unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let runner = TestRunner::new(1, None).unwrap();

        let output = "running 2 tests\ntest tests::test_one ... ok\ntest tests::test_two ... FAILED\ntest result: FAILED. 1 passed; 1 failed";
        runner.format_test_output(output);
    }

    #[test]
    #[serial_test::serial]
    fn test_run_custom_tests() {
        let temp_dir = TempDir::new().unwrap();
        let problem_dir = temp_dir.path().join("0001_test");
        fs::create_dir(&problem_dir).unwrap();
        fs::write(problem_dir.join("solution.rs"), "").unwrap();

        let test_file = temp_dir.path().join("test_cases.json");
        let test_content = r#"{
            "problem_id": "1",
            "test_cases": [
                {
                    "input": "[1,2,3]",
                    "expected": "6",
                    "explanation": "Sum of array"
                },
                {
                    "input": "[4,5]",
                    "expected": "9"
                }
            ]
        }"#;
        fs::write(&test_file, test_content).unwrap();

        let _guard = TestDirGuard::new(temp_dir);

        let runner = TestRunner::new(1, None).unwrap();
        let result = runner.run_custom_tests(&test_file);
        assert!(result.is_ok());
    }
}
