# AGENTS.md - LeetCode CLI

This document provides essential information for AI coding agents working on the LeetCode CLI project.

## Project Overview

**LeetCode CLI** is a command-line tool written in Rust for LeetCode practice. It allows users to randomly select problems, download them locally, write solutions, run tests, and submit answers directly to LeetCode.

### Key Features
- 🎲 **Random Problem Selection** - Select problems randomly by difficulty and tags
- 📥 **Local Download** - Download problem descriptions and code templates
- 🧪 **Local Testing** - Run Rust unit tests locally using Cargo
- 📤 **Submit Solutions** - Submit directly to LeetCode and view results
- 📋 **Problem List** - View all problems and their status
- 🔍 **Problem Details** - View detailed problem descriptions

## Technology Stack

| Component | Technology |
|-----------|------------|
| **Language** | Rust (Edition 2024) |
| **Toolchain** | Nightly (`nightly-2026-02-01`) |
| **Async Runtime** | Tokio |
| **HTTP Client** | Reqwest |
| **CLI Framework** | Clap v4 (derive feature) |
| **Serialization** | Serde |
| **Configuration** | Confy |
| **Terminal UI** | Colored, Indicatif |
| **Error Handling** | Anyhow, Thiserror |
| **Testing** | cargo-nextest, cargo-llvm-cov |
| **Build Tasks** | cargo-make |

## Project Structure

```
leetcode-cli/
├── src/
│   ├── main.rs              # CLI entry point (~100 lines)
│   ├── api.rs               # LeetCode API client (~370 lines)
│   ├── problem.rs           # Problem data structures (~240 lines)
│   ├── template.rs          # Code template generation (~290 lines)

│   ├── config.rs            # Configuration management (~110 lines)
│   ├── problems/            # Problem solutions (downloaded)
│   │   ├── mod.rs           # Module declarations
│   │   ├── p0001_two_sum.rs # Problem solution files

│   └── commands/            # Subcommand modules
│       ├── mod.rs           # Shared command utilities (~150 lines)
│       ├── pick.rs          # Pick random problem and download (~70 lines)
│       ├── test.rs          # Run tests (~20 lines)
│       ├── submit.rs        # Submit solution (~30 lines)
│       ├── login.rs         # Login to LeetCode (~35 lines)
│       ├── list.rs          # List problems (~80 lines)
│       └── show.rs          # Show problem details (~60 lines)
├── Cargo.toml              # Project configuration
├── Cargo.lock              # Dependency lock file
├── Makefile.toml           # cargo-make tasks
├── rust-toolchain.toml     # Rust toolchain specification
├── rustfmt.toml            # Rustfmt configuration
├── install.sh              # Installation script
├── README.md               # User documentation
├── QUICKSTART.md           # Quick start guide
├── USAGE_EXAMPLES.md       # Detailed usage examples
├── CONTRIBUTING.md         # Contribution guidelines
├── PROJECT_SUMMARY.md      # Project summary
└── .github/workflows/      # CI/CD workflows
```

## Build and Test Commands

### Basic Commands

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run tests with nextest (faster)
cargo nextest run

# Run tests with coverage
cargo llvm-cov nextest --html
```

### Code Quality Commands (via ./dev script)

```bash
# Run all checks
./dev check

# Individual checks
./dev check-fmt        # Format check
./dev check-clippy     # Lint check (denies warnings)
./dev check-typos      # Spell check
./dev check-machete    # Check for unused dependencies
./dev check-dep-sort   # Check dependency sorting
./dev test             # Run unit tests with nextest
./dev test-cov         # Run tests with coverage
./dev clean            # Clean build artifacts
```

### Installation

```bash
# Using install script
./install.sh

# Or manually
cargo build --release
cp target/release/leetcode-cli ~/.local/bin/
```

## Code Style Guidelines

### Rustfmt Configuration

The project uses a custom `rustfmt.toml` configuration:

```toml
edition = "2021"
style_edition = "2024"
comment_width = 120
format_code_in_doc_comments = true
format_macro_bodies = true
format_macro_matchers = true
normalize_comments = true
normalize_doc_attributes = true
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
reorder_impl_items = true
reorder_imports = true
tab_spaces = 4
wrap_comments = true
```

### Coding Standards

1. **Error Handling**: Use `anyhow` for application errors
2. **Documentation**: Add doc comments for public APIs
3. **Imports**: Group imports as `StdExternalCrate` (std, external, crate)
4. **Clippy**: All warnings are treated as errors in CI (`-D warnings`)

## Module Descriptions

### `main.rs`
CLI entry point using Clap derive macros. Defines 6 subcommands and dispatches to command modules.

### `commands/mod.rs`
Shared utilities for all commands:
- `prompt_input()` - Read user input from stdin
- `prompt_confirm()` - Read yes/no confirmation
- `print_problem_summary()` - Display problem overview
- `print_submission_result()` - Display submission results
- `find_solution_file()` - Locate solution file by problem ID
- `TestDirGuard` - RAII guard for test directory management

### `commands/pick.rs`
Random problem selection with optional difficulty/tag filters. When user confirms,
downloads the problem to local workspace with code templates.

### `commands/test.rs`
Run local tests for a problem. Executes `cargo test p{id:04}::` to run tests for the specific problem module.

### `commands/submit.rs`
Submit solution to LeetCode.

### `commands/login.rs`
Save LeetCode credentials.

### `commands/list.rs`
List problems with difficulty and status filters.

### `commands/show.rs`
Display detailed problem description.

### `api.rs`
LeetCode API client (`LeetCodeClient`):
- Fetches problem list from `https://leetcode.com/api/problems/all/`
- GraphQL queries for problem details
- Solution submission with polling for results (uses `backon` for retry with exponential backoff)
- Cookie-based authentication (LEETCODE_SESSION, csrftoken)

### `problem.rs`
Data structures for LeetCode problems:
- `Problem` / `ProblemList` - Basic problem info
- `ProblemDetail` - Full problem details including content
- `CodeSnippet` - Language-specific code templates
- `TestCase` - Test case structure
- HTML content cleaning utilities

### `template.rs`
Generates problem templates:
- `CodeTemplate::write_rust_template()` - Generates `src/solutions/p{id}_{slug}.rs`


### `config.rs`
Configuration management using Confy:
- Stores: session_cookie, csrf_token, default_language, workspace_path, editor
- Config location: `~/.config/leetcode-cli/config.toml`

## Testing Instructions

### Running Tests

```bash
# All tests
cargo nextest run

# With coverage report
cargo llvm-cov nextest --html
# Then open: target/llvm-cov/html/index.html
```

### Test Structure

Each module has inline tests in `#[cfg(test)]` modules:
- `problem.rs` - Tests metadata parsing
- `config.rs` - Tests config defaults and authentication check
- `template.rs` - Tests template generation


### Problem Directory Structure

When a problem is downloaded, it creates:

```
src/solutions/
├── mod.rs                    # Updated with new module declaration
└── p0001_two_sum.rs          # Rust solution with doc comments and tests
```

Solutions are stored as individual Rust modules in `src/solutions/`, with the problem description embedded as doc comments in the solution file.

## CI/CD Configuration

### GitHub Actions Workflows

Located in `.github/workflows/`:

1. **test.yml** - Runs tests on Ubuntu, macOS, Windows + coverage
2. **check.yml** - Runs fmt, clippy, and typos checks
3. **safety.yml** - Safety checks
4. **scheduled.yml** - Scheduled maintenance tasks
5. **dependency-review.yml** - Dependency security review
6. **scorecards.yml** - OpenSSF Scorecard

### CI Requirements

All PRs must pass:
- `cargo fmt --check`
- `cargo clippy --workspace --all-features --all-targets -- -D warnings`
- `cargo nextest run --locked`
- typos check

## Security Considerations

### Authentication
- LeetCode credentials stored in `~/.config/leetcode-cli/config.toml`
- Session cookies can expire; users must re-login periodically
- Credentials are never logged or transmitted except to LeetCode

### Network Security
- HTTPS only for all LeetCode API calls
- Uses standard browser User-Agent headers
- Cookie store enabled for session management

### CI Security
- Uses `step-security/harden-runner` for egress policy control
- Pinned action versions with SHA hashes
- Minimal GITHUB_TOKEN permissions

## Development Workflow

1. **Setup**: Ensure Rust nightly is installed (`rust-toolchain.toml` handles this)
2. **Code**: Make changes following Rust standards
3. **Format**: Run `./dev check-fmt`
4. **Lint**: Run `./dev check-clippy`
5. **Test**: Run `./dev test`
6. **Check**: Run `./dev check` to verify all checks pass

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `serde` / `serde_json` | Serialization |
| `anyhow` | Error handling |
| `backon` | Retry with exponential backoff |
| `confy` | Configuration management |
| `rand` | Random problem selection |
| `scraper` | HTML parsing |

## Notes for AI Agents

1. **Nightly Rust**: This project requires nightly toolchain; don't try to use stable features that conflict
2. **Clippy Strict**: CI treats all clippy warnings as errors
3. **Test with nextest**: Prefer `cargo nextest run` over `cargo test`
4. **Format on save**: Use the provided `rustfmt.toml` configuration
5. **Module structure**: Keep modules focused; main.rs is for CLI handling only
6. **Solution templates**: When modifying templates, update template.rs and check generated output in src/solutions/
