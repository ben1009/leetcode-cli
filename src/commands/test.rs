//! Test command - Run local tests for a problem

use std::process::Command;

use anyhow::Result;
use colored::*;

/// Run local tests for a problem
pub async fn execute(id: u32) -> Result<()> {
    println!("{}", format!("Running tests for problem {id}...").cyan());

    // Run tests for the specific problem module
    // Module name pattern: p0001_two_sum::
    let module_pattern = format!("p{id:04}::");

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
        format_test_output(&stdout);
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

fn format_test_output(output: &str) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_test_output_ok() {
        // This test mainly ensures format_test_output doesn't panic
        let output = "running 3 tests\ntest tests::test_one ... ok\ntest tests::test_two ... ok\ntest result: ok. 3 passed; 0 failed";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_failed() {
        let output = "running 2 tests\ntest tests::test_one ... ok\ntest tests::test_two ... FAILED\ntest result: FAILED. 1 passed; 1 failed";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_individual_tests() {
        // Test individual test status lines
        let output = "test test_a ... ok\ntest test_b ... FAILED\ntest test_c ... ok";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_running_line() {
        // Test running line
        let output = "running 10 tests";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_other_lines() {
        // Test other lines that don't match any pattern
        let output = "Some other output\nMore output\n";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_empty() {
        // Test empty output
        format_test_output("");
    }

    #[test]
    fn test_format_test_output_mixed_content() {
        // Test mixed content with all patterns
        let output = "running 5 tests\ntest tests::a ... ok\ntest tests::b ... FAILED\nsome random text\ntest result: ok. 5 passed; 0 failed";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_multiple_failed() {
        // Test with multiple failed tests
        let output = "running 5 tests\ntest tests::a ... ok\ntest tests::b ... FAILED\ntest tests::c ... FAILED\ntest tests::d ... ok\ntest result: FAILED. 2 passed; 2 failed";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_with_compiling() {
        // Test output with compiling message (should be filtered)
        let output = "Compiling mycrate v0.1.0\nFinished test [unoptimized + debuginfo]\nRunning unittests\nrunning 2 tests\ntest tests::one ... ok\ntest result: ok";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_only_running() {
        // Test with just running line and no results
        let output = "running 0 tests";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_whitespace_variations() {
        // Test various whitespace patterns
        let output = "  running 3 tests  \n  test tests::test_one ... ok  \n  test result: ok  ";
        format_test_output(output);
    }

    #[test]
    fn test_format_test_output_long_test_names() {
        // Test with long test names
        let output = "running 2 tests\ntest tests::very_long_test_name_that_describes_exactly_what_it_tests ... ok\ntest tests::another_very_long_test_name ... FAILED";
        format_test_output(output);
    }

    #[test]
    fn test_module_pattern_formatting() {
        // Verify module pattern is formatted correctly for different IDs
        let test_cases = vec![
            (1, "p0001::"),
            (42, "p0042::"),
            (999, "p0999::"),
            (1000, "p1000::"),
            (9999, "p9999::"),
        ];

        for (id, expected) in test_cases {
            let pattern = format!("p{id:04}::");
            assert_eq!(pattern, expected, "Pattern mismatch for id {}", id);
        }
    }
}
