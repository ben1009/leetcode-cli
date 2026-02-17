# Contributing to LeetCode CLI

Thank you for your interest in contributing to LeetCode CLI!

## Development Environment Setup

1. **Install Rust** (1.70+)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone Repository**
   ```bash
   git clone <repository-url>
   cd leetcode-cli
   ```

3. **Install Development Dependencies**
   ```bash
   make dev-setup
   ```

## Project Structure

```
leetcode-cli/
├── src/
│   ├── main.rs          # CLI entry and command handling
│   ├── api.rs           # LeetCode API client
│   ├── problem.rs       # Problem data structures
│   ├── template.rs      # Code template generation
│   ├── test_runner.rs   # Local test runner
│   └── config.rs        # Configuration management
├── examples/            # Example problems
├── Cargo.toml          # Project configuration
├── Makefile            # Build scripts
└── README.md           # Documentation
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
   make fmt
   ```

4. **Run Linter**
   ```bash
   make lint
   ```

5. **Run Tests**
   ```bash
   make test
   ```

6. **Build Release**
   ```bash
   make release
   ```

## Code Standards

### Rust Style

- Use `cargo fmt` to format code
- Use `cargo clippy` to check code
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)

### Error Handling

- Use `anyhow` for error handling
- Provide meaningful error messages
- Use `thiserror` to define custom error types

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
- Rust version: [e.g. 1.70.0]
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

By contributing code, you agree that your contribution will be released under the MIT License.
