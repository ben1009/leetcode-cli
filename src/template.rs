use std::{fs, path::Path};

use anyhow::Result;

use crate::problem::ProblemDetail;

pub struct CodeTemplate<'a> {
    problem: &'a ProblemDetail,
}

impl<'a> CodeTemplate<'a> {
    pub fn new(problem: &'a ProblemDetail) -> Self {
        Self { problem }
    }

    /// Generic helper to write generated content to a file.
    ///
    /// Eliminates the repetitive pattern of generating content and writing it.
    fn write_file<F>(&self, path: &Path, content_generator: F) -> Result<()>
    where
        F: FnOnce(&Self) -> String,
    {
        let content = content_generator(self);
        fs::write(path, content)?;
        Ok(())
    }

    pub fn write_rust_template(&self, path: &Path) -> Result<()> {
        self.write_file(path, Self::generate_rust_template)
    }

    pub fn write_description(&self, path: &Path) -> Result<()> {
        self.write_file(path, Self::generate_description)
    }

    pub fn write_test_cases(&self, path: &Path) -> Result<()> {
        self.write_file(path, Self::generate_test_cases_json)
    }

    pub fn write_cargo_toml(&self, path: &Path) -> Result<()> {
        self.write_file(path, Self::generate_cargo_toml)
    }

    fn generate_rust_template(&self) -> String {
        let mut template = String::new();

        // Add header comment
        template.push_str(&format!("// Problem: {}\n", self.problem.title));
        template.push_str(&format!("// Difficulty: {}\n", self.problem.difficulty));
        template.push_str(&format!(
            "// URL: https://leetcode.com/problems/{}/\n",
            self.problem.title_slug
        ));
        template.push('\n');

        // Add standard Rust boilerplate
        template.push_str("// Time Complexity: O()\n");
        template.push_str("// Space Complexity: O()\n");
        template.push('\n');

        // Add the code snippet from LeetCode
        if let Some(ref snippet) = self.problem.get_rust_snippet() {
            template.push_str(snippet);
        } else {
            // Default template if no snippet available
            template.push_str("struct Solution;\n\n");
            template.push_str("impl Solution {\n");
            template.push_str("    pub fn solve() {\n");
            template.push_str("        // TODO: Implement your solution here\n");
            template.push_str("    }\n");
            template.push_str("}\n");
        }

        // Add main function for local testing
        template.push('\n');
        template.push_str("fn main() {\n");
        template.push_str("    // Local testing\n");
        template.push_str("    let sol = Solution;\n");
        template.push_str("    // Add your test cases here\n");
        template.push_str("}\n");

        // Add test module
        template.push('\n');
        template.push_str("#[cfg(test)]\n");
        template.push_str("mod tests {\n");
        template.push_str("    use super::*;\n\n");

        // Add test cases from examples
        let test_cases = self.problem.parse_test_cases();
        for (i, tc) in test_cases.iter().enumerate() {
            template.push_str("    #[test]\n");
            template.push_str(&format!("    fn test_case_{}() {{\n", i + 1));
            template.push_str(&format!("        // Input: {}\n", tc.input));
            template.push_str(&format!("        // Expected: {}\n", tc.expected));
            template.push_str("        // TODO: Add test implementation\n");
            template.push_str("    }\n\n");
        }

        if test_cases.is_empty() {
            template.push_str("    #[test]\n");
            template.push_str("    fn test_example() {\n");
            template.push_str("        // TODO: Add your test case\n");
            template.push_str("    }\n");
        }

        template.push_str("}\n");

        template
    }

    fn generate_description(&self) -> String {
        let mut desc = String::new();

        desc.push_str(&format!("# {}\n\n", self.problem.title));
        desc.push_str(&format!("**Difficulty:** {}  \n", self.problem.difficulty));
        desc.push_str(&format!(
            "**URL:** https://leetcode.com/problems/{}  \n\n",
            self.problem.title_slug
        ));

        // Add problem content
        desc.push_str("## Description\n\n");
        desc.push_str(&self.problem.clean_content());
        desc.push_str("\n\n");

        // Add examples section
        if let Some(ref examples) = self.problem.example_testcases {
            desc.push_str("## Examples\n\n");
            for (i, line) in examples.lines().enumerate() {
                desc.push_str(&format!("### Example {}\n\n", i + 1));
                desc.push_str(&format!("```\n{}\n```\n\n", line));
            }
        }

        // Add constraints if available
        desc.push_str("## Constraints\n\n");
        desc.push_str("* TODO: Add constraints from problem description\n");
        desc.push('\n');

        // Add topic tags
        if let Some(ref tags) = self.problem.topic_tags {
            desc.push_str("## Topics\n\n");
            for tag in tags {
                desc.push_str(&format!("- {}\n", tag.name));
            }
            desc.push_str(
                "
            \n",
            );
        }

        // Add hints if available
        if let Some(ref hints) = self.problem.hints {
            if !hints.is_empty() {
                desc.push_str("## Hints\n\n");
                for (i, hint) in hints.iter().enumerate() {
                    desc.push_str(&format!("{}. {}\n\n", i + 1, hint));
                }
            }
        }

        // Add solution section
        desc.push_str("## Solution Approach\n\n");
        desc.push_str("<!-- Write your approach here -->\n\n");
        desc.push_str("### Complexity Analysis\n\n");
        desc.push_str("- **Time Complexity:** O()\n");
        desc.push_str("- **Space Complexity:** O()\n");

        desc
    }

    fn generate_test_cases_json(&self) -> String {
        let test_cases = self.problem.parse_test_cases();

        #[derive(serde::Serialize)]
        struct TestCaseFile {
            problem_id: String,
            problem_title: String,
            test_cases: Vec<TestCaseJson>,
        }

        #[derive(serde::Serialize)]
        struct TestCaseJson {
            input: String,
            expected: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            explanation: Option<String>,
        }

        let test_file = TestCaseFile {
            problem_id: self.problem.question_id.clone(),
            problem_title: self.problem.title.clone(),
            test_cases: test_cases
                .into_iter()
                .map(|tc| TestCaseJson {
                    input: tc.input,
                    expected: tc.expected,
                    explanation: tc.explanation,
                })
                .collect(),
        };

        serde_json::to_string_pretty(&test_file).unwrap_or_else(|_| "{}".to_string())
    }

    fn generate_cargo_toml(&self) -> String {
        let package_name = format!(
            "p{}_{}",
            self.problem.question_id,
            self.problem.title_slug.replace("-", "_")
        );

        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
"#,
            package_name
        )
    }

    #[allow(dead_code)]
    pub fn get_default_rust_template(&self) -> String {
        r#"// Default Rust template for LeetCode
// Use this when no snippet is available from LeetCode

struct Solution;

impl Solution {
    // Add your solution method here
    // pub fn method_name(params) -> ReturnType {
    //     // Your implementation
    // }
}

fn main() {
    // Local testing code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        // Add your test cases
    }
}
"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    fn create_test_problem() -> ProblemDetail {
        ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Given an array...</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: Some("2,7,11,15\n9\n\n3,2,4\n6".to_string()),
            sample_test_case: None,
            meta_data: None,
            code_snippets: Some(vec![
                crate::problem::CodeSnippet {
                    lang: "Rust".to_string(),
                    lang_slug: "rust".to_string(),
                    code: "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        \n    }\n}".to_string(),
                }
            ]),
            hints: Some(vec!["Use a hash map".to_string()]),
            topic_tags: Some(vec![
                crate::problem::TopicTag {
                    name: "Array".to_string(),
                    slug: "array".to_string(),
                },
                crate::problem::TopicTag {
                    name: "Hash Table".to_string(),
                    slug: "hash-table".to_string(),
                },
            ]),
        }
    }

    fn create_test_problem_no_snippets() -> ProblemDetail {
        ProblemDetail {
            question_id: "2".to_string(),
            title: "Add Two Numbers".to_string(),
            title_slug: "add-two-numbers".to_string(),
            content: "<p>Add two numbers...</p>".to_string(),
            difficulty: "Medium".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: None,
            code_snippets: None,
            hints: None,
            topic_tags: None,
        }
    }

    #[test]
    fn test_template_generation() {
        let problem = create_test_problem();
        let template = CodeTemplate::new(&problem);
        let rust_code = template.generate_rust_template();

        assert!(rust_code.contains("Two Sum"));
        assert!(rust_code.contains("impl Solution"));
        assert!(rust_code.contains("#[cfg(test)]"));
        // Test cases are now properly parsed
        assert!(rust_code.contains("test_case_1"));
        assert!(rust_code.contains("test_case_2"));
    }

    #[test]
    fn test_template_generation_no_snippets() {
        let problem = create_test_problem_no_snippets();
        let template = CodeTemplate::new(&problem);
        let rust_code = template.generate_rust_template();

        assert!(rust_code.contains("struct Solution"));
        assert!(rust_code.contains("// TODO: Implement your solution here"));
    }

    #[test]
    fn test_write_rust_template() {
        let temp_dir = TempDir::new().unwrap();
        let problem = create_test_problem();
        let template = CodeTemplate::new(&problem);
        let output_path = temp_dir.path().join("lib.rs");

        template.write_rust_template(&output_path).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("Two Sum"));
        assert!(content.contains("impl Solution"));
    }

    #[test]
    fn test_write_description() {
        let temp_dir = TempDir::new().unwrap();
        let problem = create_test_problem();
        let template = CodeTemplate::new(&problem);
        let output_path = temp_dir.path().join("README.md");

        template.write_description(&output_path).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("# Two Sum"));
        assert!(content.contains("**Difficulty:** Easy"));
        assert!(content.contains("Array"));
        assert!(content.contains("Hash Table"));
        assert!(content.contains("Use a hash map"));
    }

    #[test]
    fn test_write_test_cases() {
        let temp_dir = TempDir::new().unwrap();
        let problem = create_test_problem();
        let template = CodeTemplate::new(&problem);
        let output_path = temp_dir.path().join("test_cases.json");

        template.write_test_cases(&output_path).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("\"problem_id\": \"1\""));
        assert!(content.contains("\"problem_title\": \"Two Sum\""));
        // Test cases are now properly parsed
        assert!(content.contains("\"input\": \"2,7,11,15\""));
        assert!(content.contains("\"expected\": \"9\""));
    }

    #[test]
    fn test_write_cargo_toml() {
        let temp_dir = TempDir::new().unwrap();
        let problem = create_test_problem();
        let template = CodeTemplate::new(&problem);
        let output_path = temp_dir.path().join("Cargo.toml");

        template.write_cargo_toml(&output_path).unwrap();

        let content = fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("name = \"p1_two_sum\""));
        assert!(content.contains("edition = \"2021\""));
    }

    #[test]
    fn test_generate_description_without_hints() {
        let problem = create_test_problem_no_snippets();
        let template = CodeTemplate::new(&problem);
        let desc = template.generate_description();

        assert!(desc.contains("# Add Two Numbers"));
        assert!(desc.contains("**Difficulty:** Medium"));
        assert!(!desc.contains("## Hints"));
    }

    #[test]
    fn test_generate_test_cases_json_empty() {
        let problem = create_test_problem_no_snippets();
        let template = CodeTemplate::new(&problem);
        let json = template.generate_test_cases_json();

        assert!(json.contains("\"problem_id\": \"2\""));
        assert!(json.contains("\"test_cases\": []"));
    }

    #[test]
    fn test_get_default_rust_template() {
        let problem = create_test_problem();
        let template = CodeTemplate::new(&problem);
        let default = template.get_default_rust_template();

        assert!(default.contains("struct Solution"));
        assert!(default.contains("#[cfg(test)]"));
    }
}
