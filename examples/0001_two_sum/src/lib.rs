// Problem: Two Sum
// Difficulty: Easy
// URL: https://leetcode.com/problems/two-sum/

// Time Complexity: O(n)
// Space Complexity: O(n)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Given an array of integers nums and an integer target,
    /// return indices of the two numbers such that they add up to target.
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        
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
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
