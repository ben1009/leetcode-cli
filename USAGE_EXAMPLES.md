# Usage Examples

This document provides detailed usage examples for LeetCode CLI.

## Basic Usage

### First Time Use

```bash
# Compile and install
cargo make install

# Verify installation
leetcode-cli --version
```

### Login to LeetCode

```bash
# Interactive login (recommended)
leetcode-cli login

# Or provide credentials directly
leetcode-cli login \
  --session "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9..." \
  --csrf "your_csrf_token_here"
```

## Problem Selection

### Random Selection

```bash
# Completely random
leetcode-cli pick

# Filter by difficulty
leetcode-cli pick --difficulty easy    # Easy
leetcode-cli pick --difficulty medium  # Medium
leetcode-cli pick --difficulty hard    # Hard

# Combined filter
leetcode-cli pick -d medium
```

### Specific Problem

```bash
# Select specific problem
leetcode-cli pick --id 1

# Or
leetcode-cli pick -i 42
```

## Problem Download

When you pick a problem, it is automatically downloaded. The directory structure is:

Solutions are stored in `src/solutions/` as individual Rust modules:

```
src/solutions/
├── mod.rs                    # Module declarations (auto-generated)
├── p0001_two_sum.rs          # Problem solution with doc comments
├── p0002_add_two_numbers.rs  # Another problem

```

## Local Development

### Write Solution

```bash
# Problems are stored in src/solutions/
# Edit the problem file directly
vim src/solutions/p0001_two_sum.rs
# Or use your favorite editor
code src/solutions/p0001_two_sum.rs

# The problem description is in the doc comments at the top of the file
```

### Run Tests

```bash
# Method 1: Use CLI test command
leetcode-cli test --id 1

# Method 2: Use cargo directly with module path
cargo test p0001_two_sum

# Run specific test
cargo test test_two_sum_example_1

# Show test output
cargo test p0001_two_sum -- --nocapture
```

### Debug Code

```bash
# Add a temporary test for debugging
cargo test p0001_two_sum -- --nocapture

# Or use rust-script for quick prototyping
# See: https://github.com/fornwall/rust-script
```

## Submit Solution

### Basic Submit

```bash
# Submit current problem solution
leetcode-cli submit --id 1

# CLI will find the solution in src/solutions/p0001_*.rs
```

### Specify File Submit

```bash
# Submit specific file
leetcode-cli submit -i 1 -f ./my_solution.rs

# Or
leetcode-cli submit --id 42 --file ~/solutions/problem42.rs
```

### View Submit Result

```bash
# After submission, results like the following will be displayed
# ✓ Accepted!
#   Runtime: 4 ms (faster than 95.50%)
#   Memory: 2.3 MB (less than 85.20%)
```

## Problem Management

### View Problem List

```bash
# View all problems
leetcode-cli list

# Filter by difficulty
leetcode-cli list --difficulty easy
leetcode-cli list --difficulty medium
leetcode-cli list --difficulty hard

# Filter by status
leetcode-cli list --status solved      # Solved
leetcode-cli list --status attempting  # Attempting
leetcode-cli list --status unsolved    # Unsolved

# Combined filter
leetcode-cli list -d medium -s unsolved
```

### View Problem Details

```bash
leetcode-cli show --id 1
```

## Complete Workflows

### Workflow 1: Random Practice

```bash
# 1. Randomly select a medium difficulty problem
leetcode-cli pick -d medium

# 2. Problem auto-downloaded to src/solutions/
#    (e.g., src/solutions/p000X_problem_name.rs)

# 3. Read problem description in the doc comments at top of the file
head -50 src/solutions/p000X_problem_name.rs

# 4. Write solution (edit the problem file)
vim src/solutions/p000X_problem_name.rs

# 5. Local testing
leetcode-cli test -i X
# Or: cargo test p000X_problem_name

# 6. Submit solution
leetcode-cli submit -i X

# 7. If failed, continue modifying and repeat steps 4-6
```

### Workflow 2: Targeted Practice

```bash
# 1. View unsolved problems
leetcode-cli list -s unsolved

# 2. Select interesting problem to view details
leetcode-cli show -i 42

# 3. Pick the problem (auto-downloads)
leetcode-cli pick -i 42

# 4. Solve and test
vim src/solutions/p0042_problem_name.rs
# ... write code ...
leetcode-cli test -i 42
# Or: cargo test p0042_problem_name

# 5. Submit
leetcode-cli submit -i 42
```

### Workflow 3: Daily Challenge

```bash
#!/bin/bash
# daily_challenge.sh

echo "🎯 Getting today's challenge..."

# Randomly select a problem
leetcode-cli pick -d medium

# Get recently downloaded problem (latest by modification time)
LATEST_PROBLEM=$(ls -t src/solutions/p*.rs | head -1)

echo "📁 Today's problem: $LATEST_PROBLEM"
echo "📝 Problem description:"
head -30 "$LATEST_PROBLEM"

echo ""
echo "Start solving! Edit $LATEST_PROBLEM"
```

## Advanced Usage

### Batch Download

```bash
#!/bin/bash
# Download first 50 problems

for i in {1..50}; do
    echo "Downloading problem $i..."
    leetcode-cli pick -i $i
done

# All problems will be in src/solutions/
ls -la src/solutions/
```

### Editor Integration

```bash
# VS Code integration - open problems directory
leetcode-cli pick -d medium && code src/solutions/

# Vim integration
leetcode-cli pick && vim src/solutions/p*.rs

# Emacs integration
leetcode-cli pick && emacs src/solutions/p*.rs

# Open specific problem
vim src/solutions/p0001_two_sum.rs
```

## Troubleshooting

### Login Issues

```bash
# If login expires, re-login
leetcode-cli login

# Check current config
cat ~/.config/leetcode-cli/config.toml
```

### Test Failure

```bash
# Check Rust syntax
cargo check

# View detailed errors
cargo test -- --nocapture

# Run specific test manually
cargo test test_case_1 -- --exact
```

### Submit Failure

```bash
# Check network connection
curl -I https://leetcode.com

# Confirm logged in
leetcode-cli login

# Check code format
cargo fmt
cargo clippy
```

## Tips

1. **Use aliases to speed up workflow**
   ```bash
   alias lcp='leetcode-cli pick'
   alias lct='leetcode-cli test'
   alias lcs='leetcode-cli submit'
   ```

2. **Create solution script**
   ```bash
   # solve.sh
   ID=$1
   leetcode-cli pick -i $ID
   # Find the downloaded problem file
   PROBLEM_FILE=$(ls -t src/solutions/p*.rs | head -1)
   $EDITOR "$PROBLEM_FILE"
   ```

3. **Regularly update problem list**
   ```bash
   # LeetCode adds new problems regularly
   leetcode-cli list > all_problems.txt
   ```

4. **Track progress**
   ```bash
   # Count solved problems
   leetcode-cli list -s solved | wc -l
   ```
