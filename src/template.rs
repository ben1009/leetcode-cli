use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::problem::ProblemDetail;

pub struct CodeTemplate<'a> {
    problem: &'a ProblemDetail,
}

impl<'a> CodeTemplate<'a> {
    pub fn new(problem: &'a ProblemDetail) -> Self {
        Self { problem }
    }

    pub fn write_rust_template(&self, path: &Path) -> Result<()> {
        let template = self.generate_rust_template();
        fs::write(path, template)?;
        Ok(())
    }

    pub fn write_description(&self, path: &Path) -> Result<()> {
        let description = self.generate_description();
        fs::write(path, description)?;
        Ok(())
    }

    pub fn write_test_cases(&self, path: &Path) -> Result<()> {
        let test_cases = self.generate_test_cases_json();
        fs::write(path, test_cases)?;
        Ok(())
    }

    pub fn write_cargo_toml(&self, path: &Path) -> Result<()> {
        let cargo_toml = self.generate_cargo_toml();
        fs::write(path, cargo_toml)?;
        Ok(())
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
        template.push_str("\n");

        // Add standard Rust boilerplate
        template.push_str("// Time Complexity: O()\n");
        template.push_str("// Space Complexity: O()\n");
        template.push_str("\n");

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
        template.push_str("\n");
        template.push_str("fn main() {\n");
        template.push_str("    // Local testing\n");
        template.push_str("    let sol = Solution;\n");
        template.push_str("    // Add your test cases here\n");
        template.push_str("}\n");

        // Add test module
        template.push_str("\n");
        template.push_str("#[cfg(test)]\n");
        template.push_str("mod tests {\n");
        template.push_str("    use super::*;\n\n");

        // Add test cases from examples
        let test_cases = self.problem.parse_test_cases();
        for (i, tc) in test_cases.iter().enumerate() {
            template.push_str(&format!("    #[test]\n"));
            template.push_str(&format!("    fn test_case_{}() {{\n", i + 1));
            template.push_str(&format!("        // Input: {}\n", tc.input));
            template.push_str(&format!("        // Expected: {}\n", tc.expected));
            template.push_str(&format!("        // TODO: Add test implementation\n"));
            template.push_str(&format!("    }}\n\n"));
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
            "**URL:** https://leetcode/problems/{}  \n\n",
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
        desc.push_str("\n");

        // Add topic tags
        if let Some(ref tags) = self.problem.topic_tags {
            desc.push_str("## Topics\n\n");
            for tag in tags {
                desc.push_str(&format!("- {}\n", tag.name));
            }
            desc.push_str("\n");
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
    use super::*;

    #[test]
    fn test_template_generation() {
        let problem = ProblemDetail {
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
            hints: None,
            topic_tags: Some(vec![
                crate::problem::TopicTag {
                    name: "Array".to_string(),
                    slug: "array".to_string(),
                }
            ]),
        };

        let template = CodeTemplate::new(&problem);
        let rust_code = template.generate_rust_template();

        assert!(rust_code.contains("Two Sum"));
        assert!(rust_code.contains("impl Solution"));
        assert!(rust_code.contains("#[cfg(test)]"));
    }
}
