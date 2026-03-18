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

- **Language**: Rust (nightly toolchain)
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
│   ├── main.rs              # CLI entry (~100 lines)
│   ├── api.rs               # LeetCode API client (~370 lines)
│   ├── problem.rs           # Problem data structures (~240 lines)
│   ├── template.rs          # Code template generation (~290 lines)
│   ├── config.rs            # Configuration management (~110 lines)
│   ├── lib.rs               # Library exports
│   ├── solutions/           # Problem solutions
│   │   ├── mod.rs           # Module declarations
│   │   └── p0001_two_sum.rs # Problem solution files
│   └── commands/            # Subcommand modules
│       ├── mod.rs           # Shared utilities (~150 lines)
│       ├── pick.rs          # Pick and download (~70 lines)
│       ├── test.rs          # Run tests (~20 lines)
│       ├── submit.rs        # Submit solution (~30 lines)
│       ├── login.rs         # Login to LeetCode (~35 lines)
│       ├── list.rs          # List problems (~80 lines)
│       └── show.rs          # Show problem details (~60 lines)
├── Cargo.toml              # Project configuration
├── Makefile.toml           # cargo-make tasks
├── install.sh              # Installation script
├── README.md               # User documentation
├── QUICKSTART.md           # Quick start guide
├── USAGE_EXAMPLES.md       # Detailed usage examples
├── CONTRIBUTING.md         # Contribution guidelines
└── AGENTS.md               # Agent documentation
```

**Total Code**: ~2000+ lines of Rust code

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

#### 4. Configuration (`config.rs`)

- User configuration persistence
- Cookie management
- Workspace path settings
- Editor configuration

### CLI Commands

```bash
leetcode-cli <command>

Commands:
  pick      Pick random problem (auto-downloads)
  test      Local testing
  submit    Submit solution
  login     Login
  list      Problem list
  show      Problem details
```

### Workflow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│  Random Pick │ → │  Auto Download → │  Write Code  │
│   (pick)    │    │               │    │  (solution) │
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

# Randomly select a medium difficulty problem (auto-downloads)
leetcode-cli pick --difficulty medium

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
