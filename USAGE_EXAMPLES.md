# Usage Examples

This document provides detailed usage examples for LeetCode CLI.

## Basic Usage

### First Time Use

```bash
# Compile and install
make install

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

### Basic Download

```bash
# Download to current directory
leetcode-cli download --id 1

# This creates directory: ./0001_two_sum/
```

### Specify Output Directory

```bash
# Download to specific directory
leetcode-cli download -i 1 -o ~/leetcode/problems

# Or
leetcode-cli download --id 42 --output ./problems/array
```

### Directory Structure After Download

```
0001_two_sum/
├── src/
│   └── lib.rs         # Rust code template
├── Cargo.toml         # Project configuration
├── README.md          # Problem description
└── test_cases.json    # Test cases
```

## Local Development

### Write Solution

```bash
# Enter problem directory
cd 0001_two_sum

# Edit src/lib.rs
vim src/lib.rs
# Or use your favorite editor
code src/lib.rs
```

### Run Tests

```bash
# Method 1: Use CLI test command
leetcode-cli test --id 1

# Method 2: Use cargo directly
cargo test

# Run specific test
cargo test test_case_1

# Show test output
cargo test -- --nocapture
```

### Debug Code

```bash
# Add main function for debugging
cat >> src/lib.rs << 'EOF'

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("Result: {:?}", result);
}
EOF

# Run
cargo run
```

## Submit Solution

### Basic Submit

```bash
# Submit current problem solution
leetcode-cli submit --id 1

# CLI will find src/lib.rs in 0001_* directory
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

# 2. Problem auto-downloaded, enter directory
cd 000X_problem_name

# 3. Read README.md to understand problem
cat README.md

# 4. Write solution (edit src/lib.rs)
vim src/lib.rs

# 5. Local testing
leetcode-cli test -i X

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

# 3. Download problem
leetcode-cli download -i 42

# 4. Solve and test
cd 0042_problem_name
# ... write code ...
leetcode-cli test -i 42

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

# Get recently downloaded problem directory
LATEST_DIR=$(ls -td */ | head -1)
cd "$LATEST_DIR"

echo "📁 Today's problem directory: $(pwd)"
echo "📝 Problem description:"
head -20 README.md

echo ""
echo "Start solving! Edit src/lib.rs file"
```

## Advanced Usage

### Batch Download

```bash
#!/bin/bash
# Download first 50 problems

for i in {1..50}; do
    echo "Downloading problem $i..."
    leetcode-cli download -i $i -o ./problems
done
```

### Custom Tests

```bash
# Create custom test file
cat > custom_tests.json << 'EOF'
{
  "test_cases": [
    {
      "input": "[1,2,3]",
      "expected": "6",
      "explanation": "Edge case test"
    }
  ]
}
EOF

# Use custom tests
leetcode-cli test -i 1 --test-file custom_tests.json
```

### Editor Integration

```bash
# VS Code integration
leetcode-cli pick -d medium && code .

# Vim integration
leetcode-cli pick && vim src/lib.rs

# Emacs integration
leetcode-cli pick && emacs src/lib.rs
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
   alias lcd='leetcode-cli download'
   alias lct='leetcode-cli test'
   alias lcs='leetcode-cli submit'
   ```

2. **Create solution script**
   ```bash
   # solve.sh
   ID=$1
   leetcode-cli download -i $ID
   cd $(ls -td */ | head -1)
   $EDITOR src/lib.rs
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
