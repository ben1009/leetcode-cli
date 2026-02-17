# Project Summary

## LeetCode CLI - Rust Command Line Tool

### Project Overview

A fully functional LeetCode command line tool written in Rust. Supports random problem selection, local download, code testing, and answer submission.

### Features

| Feature | Status | Description |
|---------|--------|-------------|
| Random Pick | ✅ | Filter by difficulty and tags |
| Problem Download | ✅ | Auto-generate code templates and README |
| Local Testing | ✅ | Integrated Cargo test framework |
| Solution Submit | ✅ | Submit to LeetCode |
| Problem List | ✅ | View all problems and status |
| Problem Details | ✅ | Display full problem description |
| User Authentication | ✅ | Cookie-based login |

### Tech Stack

- **Language**: Rust 1.70+
- **Async Runtime**: Tokio
- **HTTP Client**: Reqwest
- **CLI Framework**: Clap v4
- **Serialization**: Serde
- **Configuration**: Confy
- **Terminal UI**: Colored, Indicatif
- **Error Handling**: Anyhow, Thiserror

### Project Structure

```
leetcode-cli/
├── src/
│   ├── main.rs          # CLI entry (454 lines)
│   ├── api.rs           # LeetCode API client (351 lines)
│   ├── problem.rs       # Data structures (209 lines)
│   ├── template.rs      # Code template generation (252 lines)
│   ├── test_runner.rs   # Test runner (286 lines)
│   └── config.rs        # Configuration management (105 lines)
├── examples/            # Example problems
│   └── 0001_two_sum/    # Two Sum complete example
├── Cargo.toml          # Project configuration
├── Makefile            # Build scripts
├── install.sh          # Installation script
└── docs/               # Documentation
    ├── README.md
    ├── QUICKSTART.md
    ├── USAGE_EXAMPLES.md
    └── CONTRIBUTING.md
```

**Total Code**: ~1657 lines of Rust code

### Core Modules

#### 1. API Client (`api.rs`)

- LeetCode GraphQL API wrapper
- Problem list fetching and caching
- Problem detail queries
- Solution submission and result polling
- Authentication management

#### 2. Problem Management (`problem.rs`)

- Problem data structures
- Difficulty classification
- Test case parsing
- Metadata processing

#### 3. Template Generation (`template.rs`)

- Rust code template generation
- README document generation
- Test case JSON generation
- LeetCode code snippet integration

#### 4. Test Runner (`test_runner.rs`)

- Cargo test integration
- Temporary project creation
- Test result formatting
- Custom test support

#### 5. Configuration (`config.rs`)

- User configuration persistence
- Cookie management
- Workspace path settings
- Editor configuration

### CLI Commands

```bash
leetcode-cli <command>

Commands:
  pick      Random problem selection
  download  Download problem
  test      Local testing
  submit    Submit solution
  login     Login
  list      Problem list
  show      Problem details
```

### Workflow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Random Pick │ → │   Download   │ → │  Write Code  │
│   (pick)    │    │  (download) │    │  (solution) │
└─────────────┘    └─────────────┘    └──────┬──────┘
                                             ↓
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  View Result │ ← │   Submit     │ ← │  Local Test  │
│  (result)   │    │  (submit)   │    │   (test)    │
└─────────────┘    └─────────────┘    └─────────────┘
```

### Installation

```bash
# Method 1: Use install script
./install.sh

# Method 2: Use Cargo
cargo install --path .

# Method 3: Manual build
cargo build --release
cp target/release/leetcode-cli ~/.local/bin/
```

### Usage Example

```bash
# Login
leetcode-cli login

# Randomly select a medium difficulty problem
leetcode-cli pick --difficulty medium

# Download problem
leetcode-cli download --id 1

# Local testing
leetcode-cli test --id 1

# Submit solution
leetcode-cli submit --id 1
```

### Project Highlights

1. **Complete Error Handling** - Use Anyhow and Thiserror for clear error messages
2. **Async Support** - Use Tokio for efficient async network requests
3. **User-Friendly CLI** - Use Clap for complete command line argument support
4. **Colored Output** - Use Colored for beautiful terminal output
5. **Configuration Persistence** - Use Confy for automatic config file management
6. **Code Template Generation** - Auto-generate Rust code templates with tests
7. **Local Test Integration** - Seamless integration with Cargo test framework

### TODO

- [ ] Support more programming languages (Python, C++, Java, etc.)
- [ ] Problem search functionality
- [ ] Solution notes feature
- [ ] Progress statistics and visualization
- [ ] Problem bookmark feature
- [ ] Contest mode support
- [ ] Solution viewing feature

### License

MIT License

### Contributing

Issues and Pull Requests are welcome!

---

**Project Status**: ✅ Fully functional, ready for use
