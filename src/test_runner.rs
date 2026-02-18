use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Result, anyhow};
use colored::*;

pub struct TestRunner {
    problem_id: u32,
    #[allow(dead_code)]
    test_file: Option<PathBuf>,
    problem_dir: PathBuf,
}

impl TestRunner {
    pub fn new(problem_id: u32, test_file: Option<PathBuf>) -> Result<Self> {
        // Find problem directory
        let problem_dir = Self::find_problem_directory(problem_id)?;

        Ok(Self {
            problem_id,
            test_file,
            problem_dir,
        })
    }

    fn find_problem_directory(problem_id: u32) -> Result<PathBuf> {
        let current_dir = std::env::current_dir()?;

        // Look for directory starting with problem_id
        for entry in std::fs::read_dir(&current_dir)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            if (name.starts_with(&format!("{:04}_", problem_id))
                || name.starts_with(&format!("{}_", problem_id)))
                && entry.file_type()?.is_dir()
            {
                return Ok(entry.path());
            }
        }

        // Try current directory (check for new structure: Cargo.toml + src/lib.rs)
        let cargo_toml = current_dir.join("Cargo.toml");
        let lib_rs = current_dir.join("src/lib.rs");
        if cargo_toml.exists() && lib_rs.exists() {
            return Ok(current_dir);
        }

        // Try legacy structure: solution.rs in current directory
        let solution_file = current_dir.join("solution.rs");
        if solution_file.exists() {
            return Ok(current_dir);
        }

        Err(anyhow!(
            "Could not find problem directory for problem {}. \
             Make sure you're in the problem directory or specify the path.",
            problem_id
        ))
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

#[allow(dead_code)]
// Helper function to create a simple test runner script
pub fn create_test_script(problem_dir: &Path) -> Result<()> {
    let script_content = r#"#!/bin/bash
# Test runner script for LeetCode solution

echo "Running tests for LeetCode solution..."
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust."
    exit 1
fi

# Run cargo test
cargo test --lib

# Check exit code
if [ $? -eq 0 ]; then
    echo ""
    echo "✓ All tests passed!"
    exit 0
else
    echo ""
    echo "✗ Some tests failed"
    exit 1
fi
"#;

    let script_path = problem_dir.join("test.sh");
    std::fs::write(&script_path, script_content)?;

    // Make executable on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&script_path)?.permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&script_path, perms)?;
    }

    Ok(())
}

#[allow(dead_code)]
// Create a simple Cargo.toml for standalone problem directories
pub fn create_cargo_toml(problem_dir: &Path, problem_name: &str) -> Result<()> {
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
        problem_name.replace("-", "_")
    );

    std::fs::write(problem_dir.join("Cargo.toml"), cargo_toml)?;

    // Create src directory and move solution.rs to src/lib.rs
    let src_dir = problem_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;

    let solution_file = problem_dir.join("solution.rs");
    if solution_file.exists() {
        let content = std::fs::read_to_string(&solution_file)?;
        std::fs::write(src_dir.join("lib.rs"), content)?;
        // Keep the original solution.rs as well
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    #[test]
    fn test_test_runner_creation() {
        let temp_dir = TempDir::new().unwrap();
        let problem_dir = temp_dir.path().join("0001_two_sum");
        fs::create_dir(&problem_dir).unwrap();
        fs::write(problem_dir.join("solution.rs"), "fn main() {}").unwrap();

        // Change to temp directory for the test
        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&temp_dir).unwrap();

        let runner = TestRunner::new(1, None);
        assert!(runner.is_ok());

        std::env::set_current_dir(original_dir).unwrap();
    }
}
