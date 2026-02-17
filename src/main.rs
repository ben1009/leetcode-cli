use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod api;
mod config;
mod problem;
mod template;
mod test_runner;

use api::LeetCodeClient;
use config::Config;
use problem::Problem;
use template::CodeTemplate;
use test_runner::TestRunner;

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
            pick_problem(&client, id, difficulty, tag).await?;
        }
        Commands::Download { id, output } => {
            download_problem(&client, id, output).await?;
        }
        Commands::Test { id, test_file } => {
            run_tests(id, test_file).await?;
        }
        Commands::Submit { id, file } => {
            submit_solution(&client, id, file).await?;
        }
        Commands::Login { session, csrf } => {
            login(session, csrf).await?;
        }
        Commands::List { difficulty, status } => {
            list_problems(&client, difficulty, status).await?;
        }
        Commands::Show { id } => {
            show_problem(&client, id).await?;
        }
    }

    Ok(())
}

async fn pick_problem(
    client: &LeetCodeClient,
    id: Option<u32>,
    difficulty: Option<String>,
    tag: Option<String>,
) -> Result<()> {
    println!("{}", "Fetching problems...".cyan());

    let problem = if let Some(problem_id) = id {
        client.get_problem_by_id(problem_id).await?
    } else {
        client
            .get_random_problem(difficulty.as_deref(), tag.as_deref())
            .await?
    };

    if let Some(p) = problem {
        print_problem_summary(&p);

        // Ask if user wants to download
        println!("\n{}", "Download this problem? [Y/n]".yellow());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "n" {
            download_problem(client, p.stat.question_id, PathBuf::from(".")).await?;
        }
    } else {
        println!("{}", "No problem found matching the criteria.".red());
    }

    Ok(())
}

async fn download_problem(client: &LeetCodeClient, id: u32, output: PathBuf) -> Result<()> {
    println!("{}", format!("Downloading problem {id}...").cyan());

    let problem = client
        .get_problem_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Problem not found"))?;

    let detail = client
        .get_problem_detail(&problem.stat.question_title_slug())
        .await?;

    // Create problem directory
    let problem_dir = output.join(format!(
        "{:04}_{}",
        id,
        problem.stat.question_title_slug().replace("-", "_")
    ));
    std::fs::create_dir_all(&problem_dir)?;

    // Create src directory
    let src_dir = problem_dir.join("src");
    std::fs::create_dir_all(&src_dir)?;

    // Generate code template
    let template = CodeTemplate::new(&detail);
    let code_file = src_dir.join("lib.rs");
    template.write_rust_template(&code_file)?;

    // Write Cargo.toml
    let cargo_file = problem_dir.join("Cargo.toml");
    template.write_cargo_toml(&cargo_file)?;

    // Write problem description
    let desc_file = problem_dir.join("README.md");
    template.write_description(&desc_file)?;

    // Write test cases
    let test_file = problem_dir.join("test_cases.json");
    template.write_test_cases(&test_file)?;

    println!(
        "{}",
        format!("✓ Problem downloaded to: {}", problem_dir.display()).green()
    );
    println!("  - Solution: {}", code_file.display());
    println!("  - Cargo.toml: {}", cargo_file.display());
    println!("  - Description: {}", desc_file.display());
    println!("  - Test cases: {}", test_file.display());
    println!();
    println!("{}", "To run tests:".cyan());
    println!("  cd {}", problem_dir.display());
    println!("  cargo test");

    Ok(())
}

async fn run_tests(id: u32, test_file: Option<PathBuf>) -> Result<()> {
    let runner = TestRunner::new(id, test_file)?;
    runner.run().await?;
    Ok(())
}

async fn submit_solution(client: &LeetCodeClient, id: u32, file: Option<PathBuf>) -> Result<()> {
    let solution_file = if let Some(f) = file {
        f
    } else {
        // Try to find the solution file automatically
        // First, try new structure: src/lib.rs
        let pattern = format!("{:04}_*", id);
        let entries: Vec<_> = std::fs::read_dir(".")?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_name()
                    .to_string_lossy()
                    .starts_with(&format!("{:04}_", id))
            })
            .collect();

        if entries.is_empty() {
            anyhow::bail!("Problem directory not found. Please specify with --file");
        }

        let problem_dir = entries[0].path();

        // Try new structure first: src/lib.rs
        let lib_rs = problem_dir.join("src/lib.rs");
        if lib_rs.exists() {
            lib_rs
        } else {
            // Try legacy structure: solution.rs
            let solution_rs = problem_dir.join("solution.rs");
            if solution_rs.exists() {
                solution_rs
            } else {
                anyhow::bail!("Solution file not found. Expected either src/lib.rs or solution.rs");
            }
        }
    };

    println!(
        "{}",
        format!("Submitting solution for problem {id}...").cyan()
    );
    let result = client.submit(id, &solution_file).await?;
    print_submission_result(&result);

    Ok(())
}

async fn login(session: Option<String>, csrf: Option<String>) -> Result<()> {
    let mut config = Config::load()?;

    if let Some(s) = session {
        config.session_cookie = Some(s);
    } else {
        println!("{}", "Please enter your LeetCode session cookie:".cyan());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        config.session_cookie = Some(input.trim().to_string());
    }

    if let Some(c) = csrf {
        config.csrf_token = Some(c);
    } else {
        println!("{}", "Please enter your CSRF token:".cyan());
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        config.csrf_token = Some(input.trim().to_string());
    }

    config.save()?;
    println!("{}", "✓ Login credentials saved successfully!".green());
    println!("{}", "You can now submit solutions to LeetCode.".green());

    Ok(())
}

async fn list_problems(
    client: &LeetCodeClient,
    difficulty: Option<String>,
    status: Option<String>,
) -> Result<()> {
    println!("{}", "Fetching problem list...".cyan());

    let problems = client.get_all_problems().await?;

    println!(
        "\n{:<6} {:<50} {:<10} {:<10}",
        "ID", "Title", "Difficulty", "Status"
    );
    println!("{}", "-".repeat(80));

    for problem in problems {
        let diff_str = match problem.difficulty.level {
            1 => "Easy".green(),
            2 => "Medium".yellow(),
            3 => "Hard".red(),
            _ => "Unknown".normal(),
        };

        let status_str = if problem.status == Some("ac".to_string()) {
            "✓ Solved".green()
        } else if problem.status == Some("notac".to_string()) {
            "~ Trying".yellow()
        } else {
            "○ New".normal()
        };

        if let Some(ref diff_filter) = difficulty {
            let level = match diff_filter.to_lowercase().as_str() {
                "easy" => 1,
                "medium" => 2,
                "hard" => 3,
                _ => 0,
            };
            if problem.difficulty.level != level {
                continue;
            }
        }

        if let Some(ref status_filter) = status {
            let should_show = match status_filter.to_lowercase().as_str() {
                "solved" => problem.status == Some("ac".to_string()),
                "attempting" => problem.status == Some("notac".to_string()),
                "unsolved" => problem.status.is_none(),
                _ => true,
            };
            if !should_show {
                continue;
            }
        }

        println!(
            "{:<6} {:<50} {:<10} {:<10}",
            problem.stat.question_id,
            problem
                .stat
                .question_title()
                .chars()
                .take(48)
                .collect::<String>(),
            diff_str,
            status_str
        );
    }

    Ok(())
}

async fn show_problem(client: &LeetCodeClient, id: u32) -> Result<()> {
    let problem = client
        .get_problem_by_id(id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Problem not found"))?;

    let detail = client
        .get_problem_detail(&problem.stat.question_title_slug())
        .await?;

    println!("\n{}", "═".repeat(80).cyan());
    println!(
        "{} {}. {}",
        "Problem".bold(),
        problem.stat.question_id,
        problem.stat.question_title().bold()
    );
    println!("{}", "═".repeat(80).cyan());

    let diff_str = match problem.difficulty.level {
        1 => "Easy".green(),
        2 => "Medium".yellow(),
        3 => "Hard".red(),
        _ => "Unknown".normal(),
    };
    println!("{} {}", "Difficulty:".bold(), diff_str);
    println!(
        "{} {:.1}%",
        "Acceptance Rate:".bold(),
        problem.stat.total_acs as f64 / problem.stat.total_submitted as f64 * 100.0
    );
    println!("{}", "─".repeat(80).cyan());

    // Print description
    println!(
        "\n{}",
        detail.content.replace("<p>", "").replace("</p>", "\n\n")
    );

    // Print examples if available
    if let Some(examples) = &detail.example_testcases {
        println!("{}", "Examples:".bold());
        for (i, example) in examples.lines().enumerate() {
            println!("  {} {}", format!("{}.", i + 1).cyan(), example);
        }
    }

    Ok(())
}

fn print_problem_summary(problem: &Problem) {
    println!("\n{}", "═".repeat(80).cyan());
    println!(
        "{} {}. {}",
        "✓ Found Problem".bold().green(),
        problem.stat.question_id,
        problem.stat.question_title().bold()
    );
    println!("{}", "═".repeat(80).cyan());

    let diff_str = match problem.difficulty.level {
        1 => "Easy".green(),
        2 => "Medium".yellow(),
        3 => "Hard".red(),
        _ => "Unknown".normal(),
    };

    println!("{} {}", "Difficulty:".bold(), diff_str);
    println!(
        "{} {:.1}%",
        "Acceptance Rate:".bold(),
        problem.stat.total_acs as f64 / problem.stat.total_submitted as f64 * 100.0
    );
    println!(
        "{} {}/{}",
        "Solved By:".bold(),
        problem.stat.total_acs,
        problem.stat.total_submitted
    );
    println!(
        "{} https://leetcode.com/problems/{}",
        "Link:".bold(),
        problem.stat.question_title_slug()
    );
}

fn print_submission_result(result: &api::SubmissionResult) {
    match result.status_code {
        10 => {
            println!("{}", "✓ Accepted!".green().bold());
            println!(
                "  Runtime: {} ms (faster than {:.1}%)",
                result.status_runtime, result.runtime_percentile
            );
            println!(
                "  Memory: {} MB (less than {:.1}%)",
                result.status_memory, result.memory_percentile
            );
        }
        11 => {
            println!("{}", "✗ Wrong Answer".red().bold());
            println!("  {}", result.status_msg);
            if let Some(ref output) = result.code_output {
                println!("  Your output: {}", output);
            }
            if let Some(ref expected) = result.expected_output {
                println!("  Expected: {}", expected);
            }
        }
        14 => {
            println!("{}", "✗ Time Limit Exceeded".red().bold());
        }
        15 => {
            println!("{}", "✗ Runtime Error".red().bold());
            if let Some(ref error) = result.full_runtime_error {
                println!("  {}", error);
            }
        }
        20 => {
            println!("{}", "✗ Compile Error".red().bold());
            if let Some(ref error) = result.full_compile_error {
                println!("  {}", error);
            }
        }
        _ => {
            println!("{} {}", "Status:".bold(), result.status_msg);
        }
    }
}
