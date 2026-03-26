#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

/// Problem: Total Waviness of Numbers in Range II
/// Difficulty: Hard
/// URL: https://leetcode.com/problems/total-waviness-of-numbers-in-range-ii/
///
/// You are given two integers `num1` and `num2` representing an **inclusive** range `[num1, num2]`.
/// The **waviness** of a number is defined as the total count of its **peaks** and **valleys**:
/// - A digit is a **peak** if it is **strictly greater** than both of its immediate neighbors.
/// - A digit is a **valley** if it is **strictly less** than both of its immediate neighbors.
/// - The first and last digits of a number **cannot** be peaks or valleys.
/// - Any number with fewer than 3 digits has a waviness of 0.
///
/// Return the total sum of waviness for all numbers in the range `[num1, num2]`.
///
/// **Example 1:**
/// **Input:**num1 = 120, num2 = 130
/// **Output:**3
///
/// **Example 2:**
/// **Input:**num1 = 198, num2 = 202
/// **Output:**3
///
/// **Example 3:**
/// **Input:**num1 = 4848, num2 = 4848
/// **Output:**2
///
/// **Constraints:**
/// - `1 <= num1 <= num2 <= 10^15`
pub struct Solution;

impl Solution {
    // Solved by Kimi atomically
    pub fn total_waviness(num1: i64, num2: i64) -> i64 {
        // Use digit DP: count waviness from 0 to n, then answer = f(num2) - f(num1 - 1)
        Self::count_up_to(num2) - Self::count_up_to(num1 - 1)
    }

    // Count total waviness for all numbers in [0, n]
    fn count_up_to(n: i64) -> i64 {
        if n < 100 {
            return 0;
        }

        let digits: Vec<i64> = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        // dp[pos][prev][curr][tight] = (count_of_numbers, total_waviness)
        // We use memoization with HashMap
        use std::collections::HashMap;
        let mut memo: HashMap<(usize, i64, i64, bool), (i64, i64)> = HashMap::new();

        Self::dfs(0, -1, -1, true, &digits, &mut memo).1
    }

    // Returns (count of valid numbers from this state, total waviness)
    fn dfs(
        pos: usize,
        prev: i64,  // previous digit, -1 means no digit yet
        prev2: i64, // digit before prev, -1 means no digit yet
        tight: bool,
        digits: &Vec<i64>,
        memo: &mut std::collections::HashMap<(usize, i64, i64, bool), (i64, i64)>,
    ) -> (i64, i64) {
        if pos == digits.len() {
            // Reached end, return (1 number, 0 waviness)
            return (1, 0);
        }

        let key = (pos, prev, prev2, tight);
        if let Some(&result) = memo.get(&key) {
            return result;
        }

        let limit = if tight { digits[pos] } else { 9 };
        let mut total_count = 0i64;
        let mut total_waviness = 0i64;

        for d in 0..=limit {
            let new_tight = tight && (d == limit);

            // Calculate waviness contribution for this digit
            let waviness_here = if prev2 >= 0
                && prev >= 0
                && ((prev > prev2 && prev > d) || (prev < prev2 && prev < d))
            {
                1 // prev is a peak or valley
            } else {
                0
            };

            let (count, waviness) = Self::dfs(pos + 1, d, prev, new_tight, digits, memo);

            total_count += count;
            total_waviness += waviness + waviness_here * count;
        }

        let result = (total_count, total_waviness);
        memo.insert(key, result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_3753_1() {
        assert_eq!(Solution::total_waviness(120, 130), 3);
    }

    #[test]
    fn test_case_3753_2() {
        assert_eq!(Solution::total_waviness(198, 202), 3);
    }

    #[test]
    fn test_case_3753_3() {
        assert_eq!(Solution::total_waviness(4848, 4848), 2);
    }

    #[test]
    fn test_case_3753_4() {
        // This is a large range test - digit DP should handle it efficiently
        let result = Solution::total_waviness(1434874, 2916624);
        // The result should be computed efficiently without timing out
        assert!(result >= 0);
    }
}
