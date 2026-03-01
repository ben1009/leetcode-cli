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
