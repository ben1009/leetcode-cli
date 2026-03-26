#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

/// Problem: Number of Distinct Averages
/// Difficulty: Easy
/// URL: https://leetcode.com/problems/number-of-distinct-averages/
///
/// You are given a **0-indexed** integer array `nums` of **even** length.
/// As long as `nums` is **not** empty, you must repetitively:
/// - Find the minimum number in `nums` and remove it.
/// - Find the maximum number in `nums` and remove it.
/// - Calculate the average of the two removed numbers.
///
/// The **average** of two numbers `a` and `b` is `(a + b) / 2`.
///
/// - For example, the average of `2` and `3` is `(2 + 3) / 2 = 2.5`.
///
/// Return* the number of **distinct** averages calculated using the above process*.
///
/// **Note** that when there is a tie for a minimum or maximum number, any can be removed.
///
/// **Example 1:**
/// '''
/// **Input:** nums = [4,1,4,0,3,5] **Output:** 2 **Explanation:** 1. Remove 0 and 5, and the
/// average is (0 + 5) / 2 = 2.5. Now, nums = [4,1,4,3]. 2. Remove 1 and 4. The average is (1 +
/// 4) / 2 = 2.5, and nums = [4,3]. 3. Remove 3 and 4, and the average is (3 + 4) / 2 = 3.5.
/// Since there are 2 distinct numbers among 2.5, and 3.5, we return 2.
/// '''
/// **Example 2:**
/// '''
/// **Input:** nums = [1,100] **Output:** 1 **Explanation:** There is only one average to be
/// calculated after removing 1 and 100, so we return 1.
/// '''
/// **Constraints:**
/// - `2 <= nums.length <= 100`
/// - `nums.length` is even.
/// - `0 <= nums[i] <= 100`
pub struct Solution;

impl Solution {
    // Solved by Kimi atomically
    pub fn distinct_averages(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();

        // Use a set of sums (multiplied by 2 to avoid floating point)
        // Since (a+b)/2 = (c+d)/2 iff a+b = c+d
        let mut distinct_sums = std::collections::HashSet::new();
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let sum = nums[left] + nums[right];
            distinct_sums.insert(sum);
            left += 1;
            right -= 1;
        }

        distinct_sums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_2561_1() {
        let nums = vec![4, 1, 4, 0, 3, 5];
        assert_eq!(Solution::distinct_averages(nums), 2);
    }

    #[test]
    fn test_case_2561_2() {
        let nums = vec![1, 100];
        assert_eq!(Solution::distinct_averages(nums), 1);
    }

    #[test]
    fn test_case_2561_3() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::distinct_averages(nums), 1);
    }
}
