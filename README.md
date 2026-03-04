# LeetCode CLI

[![Test](https://github.com/ben1009/leetcode-cli/workflows/Test/badge.svg)](https://github.com/ben1009/leetcode-cli/actions/workflows/test.yml)
[![codecov](https://codecov.io/gh/ben1009/leetcode-cli/branch/main/graph/badge.svg)](https://codecov.io/gh/ben1009/leetcode-cli)

A command-line tool written in Rust for LeetCode practice. Randomly select problems, download them locally, write code, and check answers.

## Test Coverage & Quality

| Metric | Status |
|--------|--------|
| **Line Coverage** | 77% (110 tests) |
| **Tests Passing** | ✅ 110/110 |
| **Clippy** | ✅ Clean |
| **Format** | ✅ Clean |

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

**How to get cookies from your browser:**

#### Method A: Using Browser Developer Tools (Recommended)

1. **Login to LeetCode** in your browser at https://leetcode.com

2. **Open Developer Tools**:
   - Chrome/Edge: Press `F12` or `Ctrl+Shift+I` (Windows/Linux), `Cmd+Option+I` (Mac)
   - Firefox: Press `F12` or `Ctrl+Shift+I` (Windows/Linux), `Cmd+Option+I` (Mac)

3. **Navigate to Application/Storage tab**:
   - Chrome/Edge: Click "Application" tab → "Cookies" → "https://leetcode.com"
   - Firefox: Click "Storage" tab → "Cookies" → "https://leetcode.com"

4. **Copy the cookie values**:
   - Find `LEETCODE_SESSION` - copy its value (long JWT string)
   - Find `csrftoken` - copy its value (shorter alphanumeric string)

5. **Paste into the CLI** when prompted or use the `--session` and `--csrf` flags

#### Method B: Using Browser Extensions

Install a cookie manager extension for easier access:
- **Chrome**: "EditThisCookie" or "Cookie-Editor"
- **Firefox**: "Cookie Quick Manager"

Then:
1. Click the extension icon while on leetcode.com
2. Find and copy `LEETCODE_SESSION` and `csrftoken`

#### Method C: Using JavaScript Console

1. Open Developer Tools (F12)
2. Go to "Console" tab
3. Run this JavaScript:
   ```javascript
   console.log('LEETCODE_SESSION:', document.cookie.match(/LEETCODE_SESSION=([^;]+)/)?.[1]);
   console.log('csrftoken:', document.cookie.match(/csrftoken=([^;]+)/)?.[1]);
   ```

**Important Security Notes:**
- These credentials are stored in `~/.config/leetcode-cli/config.toml` (Linux/Mac) or `%APPDATA%/leetcode-cli/config.toml` (Windows)
- Session expires after some time (days/weeks) - re-login when submission fails
- **Never share or commit these credentials** - treat them like passwords
- The `LEETCODE_SESSION` cookie grants access to your LeetCode account

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

### 3. Local Testing

```bash
# Run tests for a problem
leetcode-cli test --id 1
```

### 4. Submit Solution

```bash
# Submit current problem solution
leetcode-cli submit --id 1

# Specify solution file path
leetcode-cli submit --id 1 --file ./my_solution.rs
```

### 5. View Problem List

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

### 6. View Problem Details

```bash
leetcode-cli show --id 1
```

## Code Template

When downloading a problem, a Rust code template is automatically generated:

```rust
#![allow(dead_code)]

/// Problem: Two Sum
/// Difficulty: Easy
/// URL: https://leetcode.com/problems/two-sum/
///
/// [Problem description in doc comments...]
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // TODO: Implement your solution here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        // Input: nums = [2,7,11,15], target = 9
        // Expected: [0,1]
        // TODO: Add test implementation
    }
}
```

## Workflow Example

```bash
# 1. Login
leetcode-cli login

# 2. Randomly select a medium difficulty problem
leetcode-cli pick --difficulty medium

# 3. Enter problem directory and write solution
vim src/problems/p0002_add_two_numbers.rs

# 4. Local testing
leetcode-cli test --id 2

# 5. Submit solution
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
