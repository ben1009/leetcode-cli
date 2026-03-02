# TODO

## Fix Lints

- [x] Fix all clippy warnings
- [x] Add `#![warn(clippy::all)]` to main.rs
- [x] Run `cargo clippy -- -D warnings` in CI
- [x] Fix unused imports
- [x] Fix unused variables
- [x] Fix redundant clones
- [x] Fix needless borrows

## Add GitHub Actions

- [x] Create `.github/workflows/ci.yml`
- [x] Add build job (stable, beta, nightly)
- [x] Add test job
- [x] Add clippy check
- [x] Add rustfmt check
- [x] Add release workflow
- [x] Add dependabot for dependency updates

## Code Organization

- [x] Split main.rs into subcommand modules (e.g., `commands/pick.rs`, `commands/download.rs`)
- [x] Remove dead code: `check_solution_status`, `create_test_script`, `create_cargo_toml`

## Bug Fixes

- [x] Fix URL typo in `template.rs:113`: `https://leetcode/problems/` → `https://leetcode.com/problems/`

## Code Duplication

- [x] Centralize difficulty mapping: Use a shared enum/struct instead of string matching in `api.rs` and `main.rs`
  - Added `DifficultyLevel` enum with `Easy`, `Medium`, `Hard` variants
  - Added `from_str()`, `level()`, `name()` methods
  - Updated `api.rs` and `main.rs` to use the new enum
- [x] Extract input prompt helper for stdin reading pattern
  - Added `prompt_input()` helper function
  - Added `prompt_confirm()` helper function for Y/n prompts

## Unimplemented Features

- [x] Implement tag filtering in `get_random_problem()`
- [x] Use proper HTML parser for `clean_content()` (currently uses string replacement)

## Error Handling

- [x] Improve `extract_solution_code()` - brace counting could fail on edge cases
- [x] Use exponential backoff in `poll_submission_result()` instead of fixed intervals

## Testing

- [x] Improve test coverage to **76.65%** (exceeded 70% target!)
  - [x] `api.rs` - **93.98%** - Added mock HTTP tests using wiremock:
    - `fetch_all_problems`, `get_problem_by_id`, `get_random_problem`
    - `get_problem_detail`, `submit`, error handling
  - [x] `problem.rs` - **98.21%** - Stat methods, ProblemDetail methods, custom deserializer
  - [x] `config.rs` - **79.53%** - Editor/workspace getters, serde roundtrip
  - [x] `template.rs` - **95.61%** - File writing, individual generators
  - [x] `test_runner.rs` - **71.12%** - Directory finding, output formatting, custom tests
  - [x] `main.rs` - Command variants
- [x] Fix flaky `test_run_custom_tests` that changes working directory
  - Added `DirGuard` struct to ensure directory is restored on panic
- [x] Fix `parse_test_cases()` bug - splits by lines then tries to split by newlines again
  - Now correctly splits by blank lines (`\n\n`) to separate test cases

**Test Count:** 57 tests (was 5)

## Performance

- [x] Optimize `get_all_problems()` to avoid cloning entire problem list

## Documentation

- [x] Add usage examples to public APIs

## Future Refactoring Opportunities

### 1. Reduce Code Duplication in HTML Parser
- [x] Simplified `ElementRef::wrap()` call by removing unnecessary `.clone()` (Copy type)
  - The HTML parser in `html_to_markdown()` already safely handles element nodes using `if let Some(ref elem) = child_elem` pattern
  - Removed redundant `.clone()` call since `NodeRef` implements `Copy`
  - Fixed clippy warning `clone_on_copy`

### 2. Unify Directory Finding Logic
- [x] Created shared `find_problem_directory()` in `commands/mod.rs`
  - Extracted `find_problem_directories()` helper for internal use
  - Updated `test_runner.rs` to use the shared function
  - Properly handles both padded (`{:04}_`) and non-padded (`{}_`) prefixes
  - Handles current directory Cargo/solution.rs fallback

### 3. Simplify Template Writing
- [x] Added generic `write_file(path, content_generator)` helper in `template.rs`
  - Consolidates `write_rust_template()`, `write_description()`, `write_test_cases()`, `write_cargo_toml()`
  - Uses `FnOnce(&Self) -> String` for content generation

### 4. Standardize Test Setup
- [x] Extracted `TestDirGuard` helper in `commands/mod.rs`
  - RAII guard that changes to temp directory and restores on drop
  - Updated all tests in `commands/mod.rs` and `test_runner.rs` to use it
  - Eliminates repetitive directory change/restore pattern

### 5. Error Message Consistency
- [x] Standardized error message formatting across the codebase
  - **Format**: lowercase start, include context (problem ID, file path, status codes)
  - **Pattern**: `failed to <action>: <context>` for IO errors, `<what> not found: <context>` for missing items
  - Updated 14 error messages in `api.rs`, `commands/`, and `test_runner.rs`
  - Examples:
    - `problem not found: ID 123` (was: `Problem not found`)
    - `failed to fetch problem detail for 'two-sum': HTTP 404` (was: `Failed to fetch problem detail: 404`)
    - `solution file not found in '/path': expected either src/lib.rs or solution.rs`

## CI Workflow Template

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo build --release

  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo test

  clippy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - run: cargo clippy -- -D warnings

  fmt:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt -- --check
```
