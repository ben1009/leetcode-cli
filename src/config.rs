use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "leetcode-cli";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub session_cookie: Option<String>,
    pub csrf_token: Option<String>,
    pub default_language: String,
    pub workspace_path: Option<PathBuf>,
    pub editor: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            session_cookie: None,
            csrf_token: None,
            default_language: "rust".to_string(),
            workspace_path: None,
            editor: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config: Config = confy::load(APP_NAME, None)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        confy::store(APP_NAME, None, self)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn is_authenticated(&self) -> bool {
        self.session_cookie.is_some() && self.csrf_token.is_some()
    }

    #[allow(dead_code)]
    pub fn get_workspace(&self) -> PathBuf {
        self.workspace_path
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
    }

    #[allow(dead_code)]
    pub fn set_workspace(&mut self, path: PathBuf) {
        self.workspace_path = Some(path);
    }

    #[allow(dead_code)]
    pub fn get_editor(&self) -> String {
        self.editor
            .clone()
            .or_else(|| std::env::var("EDITOR").ok())
            .unwrap_or_else(|| {
                if cfg!(target_os = "windows") {
                    "notepad".to_string()
                } else if cfg!(target_os = "macos") {
                    "open".to_string()
                } else {
                    "vim".to_string()
                }
            })
    }
}

// Helper function to get config file path
#[allow(dead_code)]
pub fn get_config_path() -> Result<PathBuf> {
    let config_dir = confy::get_configuration_file_path(APP_NAME, None)?;
    Ok(config_dir)
}

// Helper function to reset config
#[allow(dead_code)]
pub fn reset_config() -> Result<()> {
    let config = Config::default();
    config.save()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_language, "rust");
        assert!(config.session_cookie.is_none());
        assert!(config.csrf_token.is_none());
        assert!(config.workspace_path.is_none());
        assert!(config.editor.is_none());
    }

    #[test]
    fn test_is_authenticated() {
        let mut config = Config::default();
        assert!(!config.is_authenticated());

        config.session_cookie = Some("test_session".to_string());
        assert!(!config.is_authenticated());

        config.csrf_token = Some("test_csrf".to_string());
        assert!(config.is_authenticated());
    }

    #[test]
    fn test_is_authenticated_only_csrf() {
        let config = Config {
            csrf_token: Some("test_csrf".to_string()),
            ..Default::default()
        };
        assert!(!config.is_authenticated());
    }

    #[test]
    fn test_get_workspace_with_path() {
        let test_path = PathBuf::from("/test/workspace");
        let config = Config {
            workspace_path: Some(test_path.clone()),
            ..Default::default()
        };
        assert_eq!(config.get_workspace(), test_path);
    }

    #[test]
    fn test_get_workspace_default() {
        let config = Config::default();
        let workspace = config.get_workspace();
        // Should return current directory when no workspace is set
        assert!(workspace.exists());
    }

    #[test]
    fn test_set_workspace() {
        let mut config = Config::default();
        let test_path = PathBuf::from("/new/workspace");
        config.set_workspace(test_path.clone());
        assert_eq!(config.workspace_path, Some(test_path));
    }

    #[test]
    fn test_get_editor_from_config() {
        let config = Config {
            editor: Some("code".to_string()),
            ..Default::default()
        };
        assert_eq!(config.get_editor(), "code");
    }

    #[test]
    #[serial_test::serial]
    fn test_get_editor_from_env() {
        // Temporarily set EDITOR env var
        let original = env::var("EDITOR").ok();
        env::set_var("EDITOR", "vim");

        let config = Config::default();
        assert_eq!(config.get_editor(), "vim");

        // Restore original value
        match original {
            Some(val) => env::set_var("EDITOR", val),
            None => env::remove_var("EDITOR"),
        }
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let config = Config {
            session_cookie: Some("session123".to_string()),
            csrf_token: Some("csrf456".to_string()),
            default_language: "python".to_string(),
            workspace_path: Some(PathBuf::from("/workspace")),
            editor: Some("emacs".to_string()),
        };

        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.session_cookie, config.session_cookie);
        assert_eq!(deserialized.csrf_token, config.csrf_token);
        assert_eq!(deserialized.default_language, config.default_language);
        assert_eq!(deserialized.workspace_path, config.workspace_path);
        assert_eq!(deserialized.editor, config.editor);
    }
}
