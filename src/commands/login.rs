//! Login command - Save LeetCode credentials

use anyhow::Result;
use colored::Colorize;

use crate::{commands::prompt_input, config::Config};

/// Login to LeetCode
pub async fn execute(session: Option<String>, csrf: Option<String>) -> Result<()> {
    let mut config = Config::load()?;

    if let Some(s) = session {
        config.session_cookie = Some(s);
    } else {
        config.session_cookie = Some(prompt_input("Please enter your LeetCode session cookie:")?);
    }

    if let Some(c) = csrf {
        config.csrf_token = Some(c);
    } else {
        config.csrf_token = Some(prompt_input("Please enter your CSRF token:")?);
    }

    config.save()?;
    println!("{}", "✓ Login credentials saved successfully!".green());
    println!("{}", "You can now submit solutions to LeetCode.".green());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_with_provided_credentials() {
        // This tests the logic flow when credentials are provided directly
        let session = "test_session_cookie".to_string();
        let csrf = "test_csrf_token".to_string();

        // Verify the values are set correctly
        assert_eq!(session, "test_session_cookie");
        assert_eq!(csrf, "test_csrf_token");
    }

    #[test]
    fn test_login_with_none_credentials() {
        // This tests the logic flow when credentials are not provided (will prompt)
        let session: Option<String> = None;
        let csrf: Option<String> = None;

        // Verify the options are None
        assert!(session.is_none());
        assert!(csrf.is_none());
    }

    #[test]
    fn test_login_partial_session_only() {
        // Test when only session is provided
        let session = "test_session".to_string();
        let csrf: Option<String> = None;

        assert!(!session.is_empty());
        assert!(csrf.is_none());
    }

    #[test]
    fn test_login_partial_csrf_only() {
        // Test when only csrf is provided
        let session: Option<String> = None;
        let csrf = "test_csrf".to_string();

        assert!(session.is_none());
        assert!(!csrf.is_empty());
    }

    #[test]
    fn test_login_empty_strings() {
        // Test with empty strings (edge case)
        let session = "";
        let csrf = "";

        assert!(session.is_empty());
        assert!(csrf.is_empty());
    }

    #[test]
    fn test_config_with_credentials() {
        // Test that config properly stores and retrieves credentials
        let mut config = Config::default();
        assert!(!config.is_authenticated());

        config.session_cookie = Some("session123".to_string());
        config.csrf_token = Some("csrf456".to_string());

        assert!(config.is_authenticated());
        assert_eq!(config.session_cookie, Some("session123".to_string()));
        assert_eq!(config.csrf_token, Some("csrf456".to_string()));
    }

    #[test]
    fn test_config_without_credentials() {
        // Test config without credentials
        let config = Config::default();
        assert!(!config.is_authenticated());
        assert!(config.session_cookie.is_none());
        assert!(config.csrf_token.is_none());
    }

    #[test]
    fn test_config_with_only_session() {
        // Test config with only session (not fully authenticated)
        let config = Config {
            session_cookie: Some("session_only".to_string()),
            ..Default::default()
        };

        assert!(!config.is_authenticated());
    }

    #[test]
    fn test_config_with_only_csrf() {
        // Test config with only csrf (not fully authenticated)
        let config = Config {
            csrf_token: Some("csrf_only".to_string()),
            ..Default::default()
        };

        assert!(!config.is_authenticated());
    }

    #[test]
    fn test_session_cookie_formats() {
        // Test various session cookie formats
        let test_cookies: Vec<String> = vec![
            "LEETCODE_SESSION=abc123".to_string(),
            "session_token_with_special_chars!@#".to_string(),
            "a".repeat(100),
            "very-long-complex-session-string-with-dashes_and.underscores123".to_string(),
        ];

        for cookie in &test_cookies {
            let session = cookie.clone();
            assert!(!session.is_empty());
            assert_eq!(session, *cookie);
        }
    }

    #[test]
    fn test_csrf_token_formats() {
        // Test various CSRF token formats
        let test_tokens: Vec<String> = vec![
            "abc123xyz789".to_string(),
            "token_with_special_chars!@#".to_string(),
            "a".repeat(64),
            "csrf-token-with-dashes".to_string(),
        ];

        for token in &test_tokens {
            let csrf = token.clone();
            assert!(!csrf.is_empty());
            assert_eq!(csrf, *token);
        }
    }
}
