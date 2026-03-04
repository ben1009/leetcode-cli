/// Problem: Two Sum
/// Difficulty: Easy
/// URL: https://leetcode.com/problems/two-sum/
///
/// Given an array of integers `nums` and an integer `target`, return *indices of the two numbers
/// such that they add up to `target`*. You may assume that each input would have ***exactly* one
/// solution**, and you may not use the *same* element twice. You can return the answer in any
/// order. **Example 1:**
/// '''
/// **Input:** nums = [2,7,11,15], target = 9 **Output:** [0,1] **Explanation:** Because nums[0] +
/// nums[1] == 9, we return [0, 1]. '''
/// **Example 2:**
/// '''
/// **Input:** nums = [3,2,4], target = 6 **Output:** [1,2]
/// '''
/// **Example 3:**
/// '''
/// **Input:** nums = [3,3], target = 6 **Output:** [0,1]
/// '''
/// **Constraints:**
/// - `2 <= nums.length <= 104`
/// - `-109 <= nums[i] <= 109`
/// - `-109 <= target <= 109`
/// - **Only one valid answer exists.**
///
/// **Follow-up: **Can you come up with an algorithm that is less than `O(n2)`time complexity?
///
/// ## Examples
///
/// ### Example 1
///
///   [2,7,11,15]
///
/// ### Example 2
///
///   9
///
/// ### Example 3
///
///   [3,2,4]
///
/// ### Example 4
///
///   6
///
/// ### Example 5
///
///   [3,3]
///
/// ### Example 6
///
///   6
#[allow(dead_code)]
pub struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index, i as i32];
            }
            map.insert(num, i as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_example_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_example_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_two_sum_example_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_no_solution() {
        let nums = vec![1, 2, 3];
        let target = 7;
        let result = Solution::two_sum(nums, target);
        assert!(result.is_empty());
    }

    #[test]
    fn test_two_sum_negative_numbers() {
        let nums = vec![-1, -2, -3, -4, -5];
        let target = -8;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![2, 4]);
    }
}
