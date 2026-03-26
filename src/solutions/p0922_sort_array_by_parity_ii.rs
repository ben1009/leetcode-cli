#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

// Solved by Kimi atomically

/// Problem: Sort Array By Parity II
/// Difficulty: Easy
/// URL: https://leetcode.com/problems/sort-array-by-parity-ii/
///
/// Given an array of integers `nums`, half of the integers in `nums` are **odd**, and the other
/// half are **even**. Sort the array so that whenever `nums[i]` is odd, `i` is **odd**, and
/// whenever `nums[i]` is even, `i` is **even**. Return *any answer array that satisfies this
/// condition*.
/// **Example 1:**
/// '''
/// **Input:** nums = [4,2,5,7] **Output:** [4,5,2,7] **Explanation:** [4,7,2,5], [2,5,4,7],
/// [2,7,4,5] would also have been accepted.
/// '''
/// **Example 2:**
/// '''
/// **Input:** nums = [2,3] **Output:** [2,3]
/// '''
/// **Constraints:**
/// - `2 <= nums.length <= 2 * 10^4`
/// - `nums.length` is even.
/// - Half of the integers in `nums` are even.
/// - `0 <= nums[i] <= 1000`
/// **Follow Up:** Could you solve it in-place?
pub struct Solution;

impl Solution {
    // Solved by Kimi atomically
    pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut even_idx = 0;
        let mut odd_idx = 1;

        for num in nums {
            if num % 2 == 0 {
                result[even_idx] = num;
                even_idx += 2;
            } else {
                result[odd_idx] = num;
                odd_idx += 2;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0958_1() {
        let nums = vec![4, 2, 5, 7];
        let result = Solution::sort_array_by_parity_ii(nums);
        // Verify even indices have even numbers
        for i in (0..result.len()).step_by(2) {
            assert_eq!(result[i] % 2, 0);
        }
        // Verify odd indices have odd numbers
        for i in (1..result.len()).step_by(2) {
            assert_eq!(result[i] % 2, 1);
        }
    }

    #[test]
    fn test_case_0958_2() {
        let nums = vec![2, 3];
        let result = Solution::sort_array_by_parity_ii(nums);
        assert_eq!(result[0] % 2, 0);
        assert_eq!(result[1] % 2, 1);
    }
}
