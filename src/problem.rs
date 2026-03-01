use scraper::{Html, Node};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct ProblemList {
    pub user_name: String,
    pub num_solved: i32,
    pub num_total: i32,
    pub ac_easy: i32,
    pub ac_medium: i32,
    pub ac_hard: i32,
    pub stat_status_pairs: Vec<Problem>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct Problem {
    pub stat: Stat,
    pub difficulty: Difficulty,
    pub paid_only: bool,
    pub is_favor: bool,
    pub frequency: i32,
    pub progress: i32,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct Stat {
    pub question_id: u32,
    #[serde(deserialize_with = "string_or_bool_option")]
    pub question__article__live: Option<String>,
    pub question__article__slug: Option<String>,
    pub question__title: Option<String>,
    pub question__title_slug: String,
    pub question__hide: bool,
    pub total_acs: i64,
    pub total_submitted: i64,
    pub frontend_question_id: u32,
    pub is_new_question: bool,
}

/// Custom deserializer that handles both string and boolean values
fn string_or_bool_option<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(Some(s)),
        Value::Bool(b) => Ok(Some(b.to_string())),
        Value::Null => Ok(None),
        _ => Err(D::Error::custom("expected string, bool, or null")),
    }
}

impl Stat {
    pub fn question_title(&self) -> String {
        self.question__title
            .clone()
            .unwrap_or_else(|| self.question__title_slug.replace("-", " "))
    }

    pub fn question_title_slug(&self) -> String {
        self.question__title_slug.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Difficulty {
    pub level: i32,
}

/// Difficulty levels for LeetCode problems
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DifficultyLevel {
    Easy = 1,
    Medium = 2,
    Hard = 3,
}

impl DifficultyLevel {
    /// Parse difficulty from string (case-insensitive)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "easy" => Some(Self::Easy),
            "medium" => Some(Self::Medium),
            "hard" => Some(Self::Hard),
            _ => None,
        }
    }

    /// Get the numeric level (1-3)
    pub fn level(self) -> i32 {
        self as i32
    }

    /// Get the display name
    #[allow(dead_code)]
    pub fn name(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Medium => "Medium",
            Self::Hard => "Hard",
        }
    }
}

impl TryFrom<i32> for DifficultyLevel {
    type Error = ();

    fn try_from(level: i32) -> Result<Self, Self::Error> {
        match level {
            1 => Ok(Self::Easy),
            2 => Ok(Self::Medium),
            3 => Ok(Self::Hard),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemDetail {
    #[serde(rename = "questionId")]
    pub question_id: String,
    pub title: String,
    #[serde(rename = "titleSlug")]
    pub title_slug: String,
    pub content: String,
    pub difficulty: String,
    #[serde(rename = "exampleTestcases")]
    pub example_testcases: Option<String>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: Option<String>,
    #[serde(rename = "metaData")]
    pub meta_data: Option<String>,
    #[serde(rename = "codeSnippets")]
    pub code_snippets: Option<Vec<CodeSnippet>>,
    pub hints: Option<Vec<String>>,
    #[serde(rename = "topicTags")]
    pub topic_tags: Option<Vec<TopicTag>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodeSnippet {
    pub lang: String,
    #[serde(rename = "langSlug")]
    pub lang_slug: String,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TopicTag {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestCase {
    pub input: String,
    pub expected: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProblemMetadata {
    #[serde(rename = "manual")]
    pub manual: bool,
    #[serde(rename = "testConfig")]
    pub test_config: Option<TestConfig>,
    #[serde(rename = "compareResult")]
    pub compare_result: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestConfig {
    #[serde(rename = "namespace")]
    pub namespace: String,
    #[serde(rename = "className")]
    pub class_name: String,
    #[serde(rename = "methodName")]
    pub method_name: String,
    #[serde(rename = "returnType")]
    pub return_type: String,
    #[serde(rename = "args")]
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Argument {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub name: String,
}

#[allow(dead_code)]
impl ProblemDetail {
    pub fn get_rust_snippet(&self) -> Option<String> {
        self.code_snippets
            .as_ref()?
            .iter()
            .find(|s| s.lang_slug == "rust")
            .map(|s| s.code.clone())
    }

    pub fn parse_metadata(&self) -> Option<ProblemMetadata> {
        self.meta_data
            .as_ref()
            .and_then(|m| serde_json::from_str(m).ok())
    }

    pub fn parse_test_cases(&self) -> Vec<TestCase> {
        let mut test_cases = Vec::new();

        if let Some(ref examples) = self.example_testcases {
            // Parse test cases from examples
            // Format: input\nexpected\n[explanation] separated by blank lines
            let blocks: Vec<&str> = examples.split("\n\n").collect();
            for block in blocks {
                let lines: Vec<&str> = block.lines().collect();
                if lines.len() >= 2 {
                    test_cases.push(TestCase {
                        input: lines[0].to_string(),
                        expected: lines[1].to_string(),
                        explanation: lines.get(2).map(|s| s.to_string()),
                    });
                }
            }
        }

        test_cases
    }

    pub fn clean_content(&self) -> String {
        html_to_markdown(&self.content)
    }
}

/// Convert HTML content to Markdown using a proper HTML parser.
///
/// This function uses the `scraper` crate to parse HTML and convert
/// common HTML elements to their Markdown equivalents.
pub fn html_to_markdown(html: &str) -> String {
    let document = Html::parse_fragment(html);
    let root = document.root_element();

    let mut output = String::new();
    let mut in_code_block = false;

    fn traverse_node(node: &scraper::ElementRef, output: &mut String, in_code_block: &mut bool) {
        for child in node.children() {
            match child.value() {
                Node::Text(text) => {
                    let text_str = text.trim();
                    if !text_str.is_empty() || *in_code_block {
                        output.push_str(&text.replace('\n', " "));
                    }
                }
                Node::Element(element) => {
                    let tag_name = element.name();
                    match tag_name {
                        "p" => {
                            if !output.is_empty() && !output.ends_with('\n') {
                                output.push('\n');
                            }
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            output.push('\n');
                        }
                        "strong" | "b" => {
                            output.push_str("**");
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            output.push_str("**");
                        }
                        "em" | "i" => {
                            output.push('*');
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            output.push('*');
                        }
                        "code" => {
                            if !*in_code_block {
                                output.push('`');
                            }
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            if !*in_code_block {
                                output.push('`');
                            }
                        }
                        "pre" => {
                            output.push_str("\n```\n");
                            *in_code_block = true;
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            *in_code_block = false;
                            output.push_str("\n```\n");
                        }
                        "ul" | "ol" => {
                            output.push('\n');
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            output.push('\n');
                        }
                        "li" => {
                            output.push_str("\n- ");
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                        }
                        "br" => {
                            output.push('\n');
                        }
                        "h1" => {
                            output.push_str("\n# ");
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            output.push('\n');
                        }
                        "h2" => {
                            output.push_str("\n## ");
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            output.push('\n');
                        }
                        "h3" => {
                            output.push_str("\n### ");
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                            output.push('\n');
                        }
                        "a" => {
                            // Extract href and text
                            if let Some(href) = element.attr("href") {
                                output.push('[');
                                traverse_node(
                                    &scraper::ElementRef::wrap(child).unwrap(),
                                    output,
                                    in_code_block,
                                );
                                output.push_str("](");
                                output.push_str(href);
                                output.push(')');
                            } else {
                                traverse_node(
                                    &scraper::ElementRef::wrap(child).unwrap(),
                                    output,
                                    in_code_block,
                                );
                            }
                        }
                        _ => {
                            // For unknown tags, just traverse children
                            traverse_node(
                                &scraper::ElementRef::wrap(child).unwrap(),
                                output,
                                in_code_block,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }

    traverse_node(&root, &mut output, &mut in_code_block);

    // Decode HTML entities
    output
        .replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&nbsp;", " ")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&#x2F;", "/")
        .replace("&#x3C;", "<")
        .replace("&#x3E;", ">")
        .replace("&#x22;", "\"")
        .replace("&#x26;", "&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_metadata_parsing() {
        let metadata_json = r#"{
            "manual": false,
            "testConfig": {
                "namespace": "leetcode",
                "className": "Solution",
                "methodName": "twoSum",
                "returnType": "Vec<i32>",
                "args": [
                    {"type": "Vec<i32>", "name": "nums"},
                    {"type": "i32", "name": "target"}
                ]
            }
        }"#;

        let metadata: ProblemMetadata = serde_json::from_str(metadata_json).unwrap();
        assert!(!metadata.manual);
        assert_eq!(metadata.test_config.unwrap().method_name, "twoSum");
    }

    #[test]
    fn test_stat_question_title_with_title() {
        let stat = Stat {
            question_id: 1,
            question__article__live: None,
            question__article__slug: None,
            question__title: Some("Two Sum".to_string()),
            question__title_slug: "two-sum".to_string(),
            question__hide: false,
            total_acs: 1000,
            total_submitted: 2000,
            frontend_question_id: 1,
            is_new_question: false,
        };
        assert_eq!(stat.question_title(), "Two Sum");
    }

    #[test]
    fn test_stat_question_title_fallback() {
        let stat = Stat {
            question_id: 1,
            question__article__live: None,
            question__article__slug: None,
            question__title: None,
            question__title_slug: "two-sum".to_string(),
            question__hide: false,
            total_acs: 1000,
            total_submitted: 2000,
            frontend_question_id: 1,
            is_new_question: false,
        };
        assert_eq!(stat.question_title(), "two sum");
    }

    #[test]
    fn test_stat_question_title_slug() {
        let stat = Stat {
            question_id: 1,
            question__article__live: None,
            question__article__slug: None,
            question__title: None,
            question__title_slug: "add-two-numbers".to_string(),
            question__hide: false,
            total_acs: 1000,
            total_submitted: 2000,
            frontend_question_id: 2,
            is_new_question: false,
        };
        assert_eq!(stat.question_title_slug(), "add-two-numbers");
    }

    #[test]
    fn test_problem_detail_get_rust_snippet() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Problem content</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: None,
            code_snippets: Some(vec![
                CodeSnippet {
                    lang: "Rust".to_string(),
                    lang_slug: "rust".to_string(),
                    code: "impl Solution {\n    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {\n        \n    }\n}".to_string(),
                },
                CodeSnippet {
                    lang: "Python".to_string(),
                    lang_slug: "python".to_string(),
                    code: "class Solution:".to_string(),
                },
            ]),
            hints: None,
            topic_tags: None,
        };

        let snippet = detail.get_rust_snippet();
        assert!(snippet.is_some());
        assert!(snippet.unwrap().contains("impl Solution"));
    }

    #[test]
    fn test_problem_detail_get_rust_snippet_none() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Problem content</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: None,
            code_snippets: Some(vec![CodeSnippet {
                lang: "Python".to_string(),
                lang_slug: "python".to_string(),
                code: "class Solution:".to_string(),
            }]),
            hints: None,
            topic_tags: None,
        };

        assert!(detail.get_rust_snippet().is_none());
    }

    #[test]
    fn test_problem_detail_get_rust_snippet_empty() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Problem content</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: None,
            code_snippets: None,
            hints: None,
            topic_tags: None,
        };

        assert!(detail.get_rust_snippet().is_none());
    }

    #[test]
    fn test_problem_detail_parse_metadata() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Problem content</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: Some(r#"{"manual": true}"#.to_string()),
            code_snippets: None,
            hints: None,
            topic_tags: None,
        };

        let metadata = detail.parse_metadata();
        assert!(metadata.is_some());
        assert!(metadata.unwrap().manual);
    }

    #[test]
    fn test_problem_detail_parse_metadata_invalid() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Problem content</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: Some("invalid json".to_string()),
            code_snippets: None,
            hints: None,
            topic_tags: None,
        };

        assert!(detail.parse_metadata().is_none());
    }

    #[test]
    fn test_problem_detail_parse_test_cases() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Problem content</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: Some("2,7,11,15\n9\n\n3,2,4\n6".to_string()),
            sample_test_case: None,
            meta_data: None,
            code_snippets: None,
            hints: None,
            topic_tags: None,
        };

        let test_cases = detail.parse_test_cases();
        assert_eq!(test_cases.len(), 2);
        assert_eq!(test_cases[0].input, "2,7,11,15");
        assert_eq!(test_cases[0].expected, "9");
        assert_eq!(test_cases[1].input, "3,2,4");
        assert_eq!(test_cases[1].expected, "6");
    }

    #[test]
    fn test_problem_detail_parse_test_cases_no_examples() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Problem content</p>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: None,
            code_snippets: None,
            hints: None,
            topic_tags: None,
        };

        let test_cases = detail.parse_test_cases();
        assert!(test_cases.is_empty());
    }

    #[test]
    fn test_problem_detail_clean_content() {
        let detail = ProblemDetail {
            question_id: "1".to_string(),
            title: "Two Sum".to_string(),
            title_slug: "two-sum".to_string(),
            content: "<p>Given <strong>nums</strong> &amp; array with &lt;elements&gt; &quot;quoted&quot;</p><ul><li>Item 1</li></ul><pre>code block</pre>".to_string(),
            difficulty: "Easy".to_string(),
            example_testcases: None,
            sample_test_case: None,
            meta_data: None,
            code_snippets: None,
            hints: None,
            topic_tags: None,
        };

        let cleaned = detail.clean_content();
        assert!(cleaned.contains("Given **nums**"));
        assert!(cleaned.contains("& array with <elements> \"quoted\""));
        assert!(cleaned.contains("- Item 1"));
        assert!(cleaned.contains("```"));
        assert!(!cleaned.contains("<p>"));
    }

    #[test]
    fn test_string_or_bool_option_with_string() {
        let json = r#"{
            "question_id": 1,
            "question__article__live": "some-string-value",
            "question__article__slug": null,
            "question__title": "Two Sum",
            "question__title_slug": "two-sum",
            "question__hide": false,
            "total_acs": 1000,
            "total_submitted": 2000,
            "frontend_question_id": 1,
            "is_new_question": false
        }"#;
        let stat: Stat = serde_json::from_str(json).unwrap();
        assert_eq!(
            stat.question__article__live,
            Some("some-string-value".to_string())
        );
    }

    #[test]
    fn test_string_or_bool_option_with_bool() {
        let json = r#"{
            "question_id": 1,
            "question__article__live": true,
            "question__article__slug": null,
            "question__title": "Two Sum",
            "question__title_slug": "two-sum",
            "question__hide": false,
            "total_acs": 1000,
            "total_submitted": 2000,
            "frontend_question_id": 1,
            "is_new_question": false
        }"#;
        let stat: Stat = serde_json::from_str(json).unwrap();
        assert_eq!(stat.question__article__live, Some("true".to_string()));
    }

    #[test]
    fn test_string_or_bool_option_with_null() {
        let json = r#"{
            "question_id": 1,
            "question__article__live": null,
            "question__article__slug": null,
            "question__title": "Two Sum",
            "question__title_slug": "two-sum",
            "question__hide": false,
            "total_acs": 1000,
            "total_submitted": 2000,
            "frontend_question_id": 1,
            "is_new_question": false
        }"#;
        let stat: Stat = serde_json::from_str(json).unwrap();
        assert_eq!(stat.question__article__live, None);
    }
}
