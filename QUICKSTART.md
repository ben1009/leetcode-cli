# Quick Start Guide

This document helps you quickly get started with LeetCode CLI.

## Installation

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Installation Steps

```bash
# 1. Clone or download project
cd leetcode-cli

# 2. Run install script
./install.sh

# Or manually install
cargo build --release
cp target/release/leetcode-cli ~/.local/bin/
```

## 5-Minute Quick Start

### Step 1: Login (1 minute)

```bash
leetcode-cli login
```

Enter when prompted:
- **Session Cookie**: Copy `LEETCODE_SESSION` from browser developer tools
- **CSRF Token**: Copy `csrftoken` from browser developer tools

**How to get cookies:**
1. Login to [LeetCode](https://leetcode.com)
2. Press F12 to open developer tools
3. Switch to Application/Storage → Cookies
4. Copy values of `LEETCODE_SESSION` and `csrftoken`

### Step 2: Random Problem Selection (1 minute)

```bash
# Random selection
leetcode-cli pick

# Or filter by difficulty
leetcode-cli pick --difficulty easy
```

After seeing the problem, enter `Y` to download.

### Step 3: Write Solution (2 minutes)

```bash
# Problems are downloaded to src/problems/
# View the problem (description is in doc comments)
head -50 src/problems/p000X_problem_name.rs

# Edit the problem file
vim src/problems/p000X_problem_name.rs  # Or use your favorite editor
```

### Step 4: Local Testing (1 minute)

```bash
# Run tests
leetcode-cli test --id X

# Or use cargo directly with module name
cargo test p000X_problem_name
```

### Step 5: Submit Solution (1 minute)

```bash
# Submit solution
leetcode-cli submit --id X
```

View results:
- ✅ **Accepted** - Correct answer!
- ❌ **Wrong Answer** - Incorrect answer, continue debugging
- ⏱️ **Time Limit Exceeded** - Timeout, need to optimize algorithm
- 💥 **Runtime Error** - Runtime error

## Common Commands

| Command | Description |
|---------|-------------|
| `leetcode-cli login` | Login |
| `leetcode-cli pick` | Random selection |
| `leetcode-cli pick -d medium` | Select medium difficulty |
| `leetcode-cli download -i 1` | Download problem 1 |
| `leetcode-cli test -i 1` | Test problem 1 |
| `leetcode-cli submit -i 1` | Submit problem 1 |
| `leetcode-cli list` | View problem list |
| `leetcode-cli show -i 1` | View problem details |

## Next Steps

- Read [Full Documentation](README.md) for more features
- View [Usage Examples](USAGE_EXAMPLES.md) for advanced usage
- Reference [Example Problem](src/problems/p0001_two_sum.rs) for solution structure

## FAQ

**Q: Is login required?**  
A: Download and local testing don't require login, but submitting solutions requires login.

**Q: What languages are supported?**  
A: Currently mainly supports Rust, but problem descriptions and test frameworks can be adapted for any language.

**Q: How to update problem list?**  
A: Problem list is automatically fetched from LeetCode API, no manual update needed.

**Q: Will Cookie expire?**  
A: Yes, Session Cookie will expire. If submission fails, please re-login.

## Get Help

```bash
# Show help
leetcode-cli --help

# Show subcommand help
leetcode-cli pick --help
leetcode-cli submit --help
```

Happy coding! 🎉
