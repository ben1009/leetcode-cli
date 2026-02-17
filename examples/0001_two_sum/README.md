# Two Sum

**Difficulty:** Easy  
**URL:** https://leetcode.com/problems/two-sum  

## Description

Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to `target`.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

## Examples

### Example 1
```
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

### Example 2
```
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

### Example 3
```
Input: nums = [3,3], target = 6
Output: [0,1]
```

## Constraints

- 2 <= nums.length <= 10^4
- -10^9 <= nums[i] <= 10^9
- -10^9 <= target <= 10^9
- Only one valid answer exists.

## Topics

- Array
- Hash Table

## Solution Approach

### Hash Map Solution

Use a hash map to store the complement of each number as we iterate through the array.

1. Create an empty hash map
2. Iterate through the array with index
3. For each number, calculate its complement (target - num)
4. If complement exists in map, return the indices
5. Otherwise, insert current number and index into map

### Complexity Analysis

- **Time Complexity:** O(n) - Single pass through the array
- **Space Complexity:** O(n) - Hash map storage

## Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_case_1
```
