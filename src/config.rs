use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.default_language, "rust");
        assert!(config.session_cookie.is_none());
        assert!(config.csrf_token.is_none());
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
}
