# LeetCode CLI

A command-line tool written in Rust for LeetCode practice. Randomly select problems, download them locally, write code, and check answers.

## Features

- 🎲 **Random Problem Selection** - Select problems randomly by difficulty and tags
- 📥 **Local Download** - Download problem descriptions and code templates
- 🧪 **Local Testing** - Run Rust unit tests locally
- 📤 **Submit Solutions** - Submit directly to LeetCode and view results
- 📋 **Problem List** - View all problems and their status
- 🔍 **Problem Details** - View detailed problem descriptions

## Installation

### Build from Source

```bash
# Clone the repository
git clone <repository-url>
cd leetcode-cli

# Build release version
cargo build --release

# Add binary to PATH
cp target/release/leetcode-cli ~/.local/bin/
```

### Requirements

- Rust 1.70+
- Cargo

## Usage

### 1. Login to LeetCode

To submit solutions, you need to login to LeetCode to get session cookie and CSRF token:

```bash
# Method 1: Interactive login
leetcode-cli login

# Method 2: Provide credentials directly
leetcode-cli login --session "your_session_cookie" --csrf "your_csrf_token"
```

**How to get cookies:**
1. Login to LeetCode website
2. Open browser developer tools (F12)
3. Switch to Application/Storage tab
4. Find Cookies > https://leetcode.com
5. Copy values of `LEETCODE_SESSION` and `csrftoken`

### 2. Random Problem Selection

```bash
# Completely random
leetcode-cli pick

# Filter by difficulty
leetcode-cli pick --difficulty easy
leetcode-cli pick --difficulty medium
leetcode-cli pick --difficulty hard

# Specify problem ID
leetcode-cli pick --id 1
```

### 3. Download Problem

```bash
# Download specific problem to current directory
leetcode-cli download --id 1

# Download to specific directory
leetcode-cli download --id 1 --output ./problems
```

After download, the following directory structure is created:
```
0001_two_sum/
├── src/
│   └── lib.rs       # Code template
├── Cargo.toml       # Project configuration
├── README.md        # Problem description
└── test_cases.json  # Test cases
```

### 4. Local Testing

```bash
# Run tests in problem directory
leetcode-cli test --id 1

# Use custom test file
leetcode-cli test --id 1 --test-file custom_tests.json
```

### 5. Submit Solution

```bash
# Submit current problem solution
leetcode-cli submit --id 1

# Specify solution file path
leetcode-cli submit --id 1 --file ./my_solution.rs
```

### 6. View Problem List

```bash
# View all problems
leetcode-cli list

# Filter by difficulty
leetcode-cli list --difficulty medium

# Filter by status
leetcode-cli list --status solved      # Solved
leetcode-cli list --status attempting  # Attempting
leetcode-cli list --status unsolved    # Unsolved
```

### 7. View Problem Details

```bash
leetcode-cli show --id 1
```

## Code Template

When downloading a problem, a Rust code template is automatically generated:

```rust
// Problem: Two Sum
// Difficulty: Easy
// URL: https://leetcode.com/problems/two-sum/

// Time Complexity: O()
// Space Complexity: O()

// LeetCode provided code snippet
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // TODO: Implement your solution
    }
}

// Main function for local testing
fn main() {
    // Local testing code
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        // Test case
    }
}
```

## Workflow Example

```bash
# 1. Login
leetcode-cli login

# 2. Randomly select a medium difficulty problem
leetcode-cli pick --difficulty medium

# 3. Download problem (if not auto-downloaded)
leetcode-cli download --id 2

# 4. Enter problem directory and write solution
cd 0002_add_two_numbers
vim src/lib.rs

# 5. Local testing
leetcode-cli test --id 2

# 6. Submit solution
leetcode-cli submit --id 2
```

## Configuration

Configuration file location:
- Linux/macOS: `~/.config/leetcode-cli/config.toml`
- Windows: `%APPDATA%/leetcode-cli/config.toml`

Example configuration:
```toml
session_cookie = "your_session_cookie"
csrf_token = "your_csrf_token"
default_language = "rust"
workspace_path = "/path/to/your/workspace"
editor = "vim"
```

## Command Cheat Sheet

| Command | Description |
|---------|-------------|
| `leetcode-cli login` | Login to LeetCode |
| `leetcode-cli pick` | Random problem selection |
| `leetcode-cli pick -d medium` | Random medium difficulty problem |
| `leetcode-cli download -i 1` | Download problem |
| `leetcode-cli test -i 1` | Local testing |
| `leetcode-cli submit -i 1` | Submit solution |
| `leetcode-cli list` | View problem list |
| `leetcode-cli show -i 1` | View problem details |

## Notes

1. **Session Cookie Security**: Login credentials are saved in local config file, please keep it safe
2. **Submission Rate Limit**: LeetCode has submission rate limits, too frequent submissions may be temporarily restricted
3. **Network Connection**: Make sure environment variables are set correctly when using proxy

## Troubleshooting

### Cannot fetch problem list
- Check network connection
- Confirm access to https://leetcode.com

### Submit failed
- Confirm correct login
- Check if session cookie has expired (need to re-login)
- Confirm correct code format

### Test run failed
- Ensure Rust and Cargo are installed
- Check if solution.rs file has syntax errors

## License

MIT License

## Contributing

Issues and Pull Requests are welcome!
