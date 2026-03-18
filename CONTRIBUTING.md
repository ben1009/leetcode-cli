# Contributing to LeetCode CLI

Thank you for your interest in contributing to LeetCode CLI!

## Development Environment Setup

1. **Install Rust** (nightly toolchain required)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup toolchain install nightly-2026-02-01
   ```

2. **Clone Repository**
   ```bash
   git clone <repository-url>
   cd leetcode-cli
   ```

3. **Development Dependencies**
   Required development dependencies like `cargo-nextest` are installed automatically
   by the `./dev` script when needed (e.g. when running `./dev test`).
   No manual installation steps are required.

## Project Structure

```
leetcode-cli/
├── src/
│   ├── main.rs              # CLI entry and command handling
│   ├── api.rs               # LeetCode API client
│   ├── problem.rs           # Problem data structures
│   ├── template.rs          # Code template generation
│   ├── config.rs            # Configuration management
│   ├── solutions/           # Problem solutions
│   │   ├── mod.rs           # Module declarations
│   │   └── p0001_two_sum.rs # Problem solution files
│   └── commands/            # Subcommand modules
│       ├── mod.rs           # Shared utilities
│       ├── pick.rs          # Pick random problem
│       ├── test.rs          # Run tests
│       ├── submit.rs        # Submit solution
│       ├── login.rs         # Login to LeetCode
│       ├── list.rs          # List problems
│       └── show.rs          # Show problem details
├── Cargo.toml              # Project configuration
├── Makefile.toml           # cargo-make tasks
├── install.sh              # Installation script
├── README.md               # User documentation
├── QUICKSTART.md           # Quick start guide
├── USAGE_EXAMPLES.md       # Detailed usage examples
└── AGENTS.md               # Agent documentation
```

## Development Workflow

1. **Create Branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Write Code**
   - Follow Rust coding standards
   - Add appropriate error handling
   - Write documentation comments

3. **Format Code**
   ```bash
   ./dev check-fmt
   ```

4. **Run Linter**
   ```bash
   ./dev check-clippy
   ```

5. **Run Tests**
   ```bash
   ./dev test
   ```

6. **Run All Checks**
   ```bash
   ./dev check
   ```
   > **Note:** This command may modify files to fix formatting issues.

7. **Build Release**
   ```bash
   cargo build --release
   ```

## Code Standards

### Rust Style

- Use `./dev check-fmt` to format code
- Use `./dev check-clippy` to check code
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Error Handling

- Use `anyhow` for error handling
- Provide meaningful error messages

### Documentation

- Add documentation comments for public APIs
- Update usage instructions in README
- Add inline comments for complex logic

## Submitting Pull Request

1. Ensure all tests pass
2. Update relevant documentation
3. Describe changes in PR description
4. Link related issues (if any)

## Reporting Bugs

Please use the following template to report bugs:

```markdown
**Bug Description**
Clear description of the bug

**Reproduction Steps**
1. Run '...'
2. Enter '...'
3. See error

**Expected Behavior**
Clear description of expected behavior

**Environment**
- OS: [e.g. macOS, Linux]
- Rust version: [e.g. nightly-2026-02-01]
- Version: [e.g. 0.1.0]

**Additional Info**
Any other relevant information
```

## Feature Requests

New feature suggestions are welcome! Please describe:
- Purpose of the feature
- Expected usage
- Possible implementation approach

## License

By contributing code, you agree that your contribution will be released under the Apache License 2.0.
