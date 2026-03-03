use std::{
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Result;
use colored::*;

pub struct TestRunner {
    problem_id: u32,
    #[allow(dead_code)]
    test_file: Option<PathBuf>,
}

impl TestRunner {
    pub fn new(problem_id: u32, test_file: Option<PathBuf>) -> Result<Self> {
        Ok(Self {
            problem_id,
            test_file,
        })
    }

    pub async fn run(&self) -> Result<()> {
        println!(
            "{}",
            format!("Running tests for problem {}...", self.problem_id).cyan()
        );

        // Run tests for the specific problem module
        // Module name pattern: p0001_two_sum::
        let module_pattern = format!("p{:04}::", self.problem_id);

        println!("{}", "Running cargo test...".cyan());

        let output = Command::new("cargo")
            .arg("test")
            .arg(&module_pattern)
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
    use super::*;

    #[test]
    fn test_test_runner_creation() {
        let runner = TestRunner::new(1, None);
        assert!(runner.is_ok());
    }

    #[test]
    fn test_format_test_output_ok() {
        let runner = TestRunner::new(1, None).unwrap();

        // This test mainly ensures format_test_output doesn't panic
        let output = "running 3 tests\ntest tests::test_one ... ok\ntest tests::test_two ... ok\ntest result: ok. 3 passed; 0 failed";
        runner.format_test_output(output);
    }

    #[test]
    fn test_format_test_output_failed() {
        let runner = TestRunner::new(1, None).unwrap();

        let output = "running 2 tests\ntest tests::test_one ... ok\ntest tests::test_two ... FAILED\ntest result: FAILED. 1 passed; 1 failed";
        runner.format_test_output(output);
    }

    #[test]
    fn test_run_custom_tests() {
        use std::fs;

        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
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

        let runner = TestRunner::new(1, None).unwrap();
        let result = runner.run_custom_tests(&test_file);
        assert!(result.is_ok());
    }
}
