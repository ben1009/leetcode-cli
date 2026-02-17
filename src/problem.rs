use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
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
pub struct Argument {
    #[serde(rename = "type")]
    pub arg_type: String,
    pub name: String,
}

impl ProblemDetail {
    pub fn get_rust_snippet(&self) -> Option<String> {
        self.code_snippets.as_ref()?.iter()
            .find(|s| s.lang_slug == "rust")
            .map(|s| s.code.clone())
    }

    pub fn parse_metadata(&self) -> Option<ProblemMetadata> {
        self.meta_data.as_ref()
            .and_then(|m| serde_json::from_str(m).ok())
    }

    pub fn parse_test_cases(&self) -> Vec<TestCase> {
        let mut test_cases = Vec::new();
        
        if let Some(ref examples) = self.example_testcases {
            for line in examples.lines() {
                // Parse input and expected output from example
                // Format varies by problem
                let parts: Vec<&str> = line.split('\n').collect();
                if parts.len() >= 2 {
                    test_cases.push(TestCase {
                        input: parts[0].to_string(),
                        expected: parts[1].to_string(),
                        explanation: parts.get(2).map(|s| s.to_string()),
                    });
                }
            }
        }

        test_cases
    }

    pub fn clean_content(&self) -> String {
        self.content
            .replace("<p>", "")
            .replace("</p>", "\n\n")
            .replace("<strong>", "**")
            .replace("</strong>", "**")
            .replace("<em>", "*")
            .replace("</em>", "*")
            .replace("<code>", "`")
            .replace("</code>", "`")
            .replace("<pre>", "```\n")
            .replace("</pre>", "\n```")
            .replace("<ul>", "")
            .replace("</ul>", "")
            .replace("<li>", "- ")
            .replace("</li>", "")
            .replace("&quot;", "\"")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&amp;", "&")
    }
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
}
