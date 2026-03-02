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
- [ ] Extract helper for repeated `scraper::ElementRef::wrap(child).unwrap()` pattern in `problem.rs`
  - 13 occurrences across the `html_to_markdown()` function
  - Could add: `fn as_element(node: &NodeRef) -> Option<ElementRef>`

### 2. Unify Directory Finding Logic
- [ ] Share `find_problem_by_id()` helper between `test_runner.rs` and `commands/mod.rs`
  - Both traverse directories with similar patterns
  - Different prefixes handled (`{:04}_` vs `{}_`)
  - Both check for Cargo structure vs legacy structure

### 3. Simplify Template Writing
- [ ] Generic `write_file(path, content)` helper for `template.rs`
  - Four functions follow identical pattern: `write_rust_template()`, `write_description()`, `write_test_cases()`, `write_cargo_toml()`
  - Only difference is the content generator method called

### 4. Standardize Test Setup
- [ ] Extract shared temp directory setup pattern for tests
  - Common pattern: `let temp_dir = TempDir::new().unwrap()` + `let original_dir = std::env::current_dir().unwrap()`
  - Used in `commands/mod.rs`, `test_runner.rs`, `api.rs`
  - Could use a `TestDirGuard` helper similar to `DirGuard`

### 5. Error Message Consistency
- [ ] Audit and standardize error message formatting
  - Some use `format!()`, others use string literals with variables
  - Some include problem IDs, others don't
  - Standardize on including context (problem ID, file path, etc.)

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
