use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};
use leetcode_cli::{api::LeetCodeClient, commands, config::Config};

#[derive(Parser)]
#[command(name = "leetcode-cli")]
#[command(about = "A CLI tool for LeetCode practice")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Pick a random problem or specific problem by ID
    Pick {
        /// Problem ID (optional, random if not specified)
        #[arg(short, long)]
        id: Option<u32>,
        /// Difficulty filter (easy, medium, hard)
        #[arg(short, long)]
        difficulty: Option<String>,
        /// Tag/Category filter
        #[arg(short, long)]
        tag: Option<String>,
    },
    /// Run local tests
    Test {
        /// Problem ID
        id: u32,
    },
    /// Submit solution to LeetCode
    Submit {
        /// Problem ID
        id: u32,
        /// Solution file path
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    /// Login to LeetCode
    Login {
        /// Session cookie from browser
        #[arg(short, long)]
        session: Option<String>,
        /// CSRF token from browser
        #[arg(short, long)]
        csrf: Option<String>,
    },
    /// List all problems
    List {
        /// Filter by difficulty
        #[arg(short, long)]
        difficulty: Option<String>,
        /// Filter by status (solved, attempting, unsolved)
        #[arg(short, long)]
        status: Option<String>,
    },
    /// Show problem details
    Show {
        /// Problem ID
        id: u32,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::load()?;
    let client = LeetCodeClient::new(config).await?;

    match cli.command {
        Commands::Pick {
            id,
            difficulty,
            tag,
        } => {
            commands::pick::execute(&client, id, difficulty, tag).await?;
        }
        Commands::Test { id } => {
            commands::test::execute(id).await?;
        }
        Commands::Submit { id, file } => {
            commands::submit::execute(&client, id, file).await?;
        }
        Commands::Login { session, csrf } => {
            commands::login::execute(session, csrf).await?;
        }
        Commands::List { difficulty, status } => {
            commands::list::execute(&client, difficulty, status).await?;
        }
        Commands::Show { id } => {
            commands::show::execute(&client, id).await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn test_cli_command_factory() {
        // Verify CLI parses correctly using clap's built-in verification
        Cli::command().debug_assert();
    }

    #[test]
    fn test_commands_display() {
        // Verify command variants exist and have proper descriptions
        let pick = Commands::Pick {
            id: Some(1),
            difficulty: Some("easy".to_string()),
            tag: Some("array".to_string()),
        };
        // Just ensure it compiles and runs
        drop(pick);

        let test = Commands::Test { id: 1 };
        drop(test);

        let submit = Commands::Submit { id: 1, file: None };
        drop(submit);

        let login = Commands::Login {
            session: None,
            csrf: None,
        };
        drop(login);

        let list = Commands::List {
            difficulty: None,
            status: None,
        };
        drop(list);

        let show = Commands::Show { id: 1 };
        drop(show);
    }

    #[test]
    fn test_pick_command_variants() {
        // Test pick with all options
        let pick_full = Commands::Pick {
            id: Some(42),
            difficulty: Some("hard".to_string()),
            tag: Some("dynamic-programming".to_string()),
        };
        match pick_full {
            Commands::Pick {
                id,
                difficulty,
                tag,
            } => {
                assert_eq!(id, Some(42));
                assert_eq!(difficulty, Some("hard".to_string()));
                assert_eq!(tag, Some("dynamic-programming".to_string()));
            }
            _ => panic!("Expected Pick command"),
        }

        // Test pick with no options (random)
        let pick_random = Commands::Pick {
            id: None,
            difficulty: None,
            tag: None,
        };
        match pick_random {
            Commands::Pick {
                id,
                difficulty,
                tag,
            } => {
                assert!(id.is_none());
                assert!(difficulty.is_none());
                assert!(tag.is_none());
            }
            _ => panic!("Expected Pick command"),
        }
    }

    #[test]
    fn test_test_command() {
        let test = Commands::Test { id: 123 };
        match test {
            Commands::Test { id } => assert_eq!(id, 123),
            _ => panic!("Expected Test command"),
        }
    }

    #[test]
    fn test_submit_command_variants() {
        // Test submit with file path
        let submit_with_file = Commands::Submit {
            id: 1,
            file: Some(PathBuf::from("src/problems/p0001_two_sum.rs")),
        };
        match submit_with_file {
            Commands::Submit { id, file } => {
                assert_eq!(id, 1);
                assert_eq!(file, Some(PathBuf::from("src/problems/p0001_two_sum.rs")));
            }
            _ => panic!("Expected Submit command"),
        }

        // Test submit without file path
        let submit_without_file = Commands::Submit { id: 2, file: None };
        match submit_without_file {
            Commands::Submit { id, file } => {
                assert_eq!(id, 2);
                assert!(file.is_none());
            }
            _ => panic!("Expected Submit command"),
        }
    }

    #[test]
    fn test_login_command_variants() {
        // Test login with provided credentials
        let login_with_creds = Commands::Login {
            session: Some("session123".to_string()),
            csrf: Some("csrf456".to_string()),
        };
        match login_with_creds {
            Commands::Login { session, csrf } => {
                assert_eq!(session, Some("session123".to_string()));
                assert_eq!(csrf, Some("csrf456".to_string()));
            }
            _ => panic!("Expected Login command"),
        }

        // Test login without credentials (will prompt)
        let login_prompt = Commands::Login {
            session: None,
            csrf: None,
        };
        match login_prompt {
            Commands::Login { session, csrf } => {
                assert!(session.is_none());
                assert!(csrf.is_none());
            }
            _ => panic!("Expected Login command"),
        }
    }

    #[test]
    fn test_list_command_variants() {
        // Test list with all filters
        let list_filtered = Commands::List {
            difficulty: Some("medium".to_string()),
            status: Some("solved".to_string()),
        };
        match list_filtered {
            Commands::List { difficulty, status } => {
                assert_eq!(difficulty, Some("medium".to_string()));
                assert_eq!(status, Some("solved".to_string()));
            }
            _ => panic!("Expected List command"),
        }

        // Test list without filters
        let list_all = Commands::List {
            difficulty: None,
            status: None,
        };
        match list_all {
            Commands::List { difficulty, status } => {
                assert!(difficulty.is_none());
                assert!(status.is_none());
            }
            _ => panic!("Expected List command"),
        }
    }

    #[test]
    fn test_show_command() {
        let show = Commands::Show { id: 999 };
        match show {
            Commands::Show { id } => assert_eq!(id, 999),
            _ => panic!("Expected Show command"),
        }
    }
}
