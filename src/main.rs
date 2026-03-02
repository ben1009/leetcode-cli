use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

mod api;
mod commands;
mod config;
mod problem;
mod template;
mod test_runner;

use api::LeetCodeClient;
use config::Config;

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
    /// Download problem to local workspace
    Download {
        /// Problem ID
        id: u32,
        /// Output directory
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
    /// Run local tests
    Test {
        /// Problem ID
        id: u32,
        /// Test case file
        #[arg(short, long)]
        test_file: Option<PathBuf>,
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
        Commands::Download { id, output } => {
            commands::download::execute(&client, id, output).await?;
        }
        Commands::Test { id, test_file } => {
            commands::test::execute(id, test_file).await?;
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
    use super::*;

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

        let download = Commands::Download {
            id: 1,
            output: PathBuf::from("."),
        };
        drop(download);

        let test = Commands::Test {
            id: 1,
            test_file: None,
        };
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
}
