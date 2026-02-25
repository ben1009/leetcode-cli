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
| **Language** | Rust (Edition 2021) |
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
│   ├── main.rs          # CLI entry point and command handling (~500 lines)
│   ├── api.rs           # LeetCode API client (~370 lines)
│   ├── problem.rs       # Problem data structures (~240 lines)
│   ├── template.rs      # Code template generation (~290 lines)
│   ├── test_runner.rs   # Local test runner (~350 lines)
│   └── config.rs        # Configuration management (~110 lines)
├── examples/            # Example problems
│   └── 0001_two_sum/    # Two Sum complete example
├── Cargo.toml          # Project configuration
├── Cargo.lock          # Dependency lock file
├── Makefile.toml       # cargo-make tasks
├── rust-toolchain.toml # Rust toolchain specification
├── rustfmt.toml        # Rustfmt configuration
├── install.sh          # Installation script
├── README.md           # User documentation
├── QUICKSTART.md       # Quick start guide
├── USAGE_EXAMPLES.md   # Detailed usage examples
├── CONTRIBUTING.md     # Contribution guidelines
├── PROJECT_SUMMARY.md  # Project summary
└── .github/workflows/  # CI/CD workflows
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

### Code Quality Commands (via cargo-make)

```bash
# Run all checks
makers check

# Individual checks
makers check-fmt        # Format check
makers check-clippy     # Lint check (denies warnings)
makers check-typos      # Spell check
makers check-machete    # Check for unused dependencies
makers check-dep-sort   # Check dependency sorting
makers test             # Run unit tests with nextest
makers test-cov         # Run tests with coverage
makers clean            # Clean build artifacts
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

1. **Error Handling**: Use `anyhow` for application errors and `thiserror` for custom error types
2. **Documentation**: Add doc comments for public APIs
3. **Imports**: Group imports as `StdExternalCrate` (std, external, crate)
4. **Clippy**: All warnings are treated as errors in CI (`-D warnings`)

## Module Descriptions

### `main.rs`
CLI entry point using Clap derive macros. Defines 7 subcommands:
- `pick` - Random problem selection with optional difficulty/tag filters
- `download` - Download problem to local directory
- `test` - Run local tests for a problem
- `submit` - Submit solution to LeetCode
- `login` - Save LeetCode credentials
- `list` - List problems with filters
- `show` - Display problem details

### `api.rs`
LeetCode API client (`LeetCodeClient`):
- Fetches problem list from `https://leetcode.com/api/problems/all/`
- GraphQL queries for problem details
- Solution submission with polling for results
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
- `CodeTemplate::write_rust_template()` - Generates `src/lib.rs`
- `CodeTemplate::write_cargo_toml()` - Generates `Cargo.toml`
- `CodeTemplate::write_description()` - Generates `README.md`
- `CodeTemplate::write_test_cases()` - Generates `test_cases.json`

### `test_runner.rs`
Local test execution:
- `TestRunner::find_problem_directory()` - Locates problem by ID
- `TestRunner::run_cargo_test_in_dir()` - Runs `cargo test` in problem dir
- Supports both new structure (`src/lib.rs`) and legacy (`solution.rs`)

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
- `test_runner.rs` - Tests test runner creation (uses tempfile)

### Problem Directory Structure

When a problem is downloaded, it creates:

```
0001_two_sum/
├── src/
│   └── lib.rs         # Rust solution with tests
├── Cargo.toml         # Project config
├── README.md          # Problem description
└── test_cases.json    # Test cases in JSON format
```

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
3. **Format**: Run `makers check-fmt`
4. **Lint**: Run `makers check-clippy`
5. **Test**: Run `makers test`
6. **Check**: Run `makers check` to verify all checks pass

## Key Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `tokio` | Async runtime |
| `reqwest` | HTTP client |
| `serde` / `serde_json` | Serialization |
| `anyhow` | Error handling |
| `colored` | Terminal colors |
| `confy` | Configuration management |
| `rand` | Random problem selection |
| `handlebars` | Template engine (available but unused) |
| `scraper` | HTML parsing (available but unused) |

## Notes for AI Agents

1. **Nightly Rust**: This project requires nightly toolchain; don't try to use stable features that conflict
2. **Clippy Strict**: CI treats all clippy warnings as errors
3. **Test with nextest**: Prefer `cargo nextest run` over `cargo test`
4. **Format on save**: Use the provided `rustfmt.toml` configuration
5. **Module structure**: Keep modules focused; main.rs is for CLI handling only
6. **Problem templates**: When modifying templates in `template.rs`, regenerate the corresponding examples in the `examples/` directory.
