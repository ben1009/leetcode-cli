#![allow(dead_code)]

/// Problem: Reverse Integer
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/reverse-integer/
///
/// Given a signed 32-bit integer `x`, return `x`* with its digits reversed*. If reversing `x`
/// causes the value to go outside the signed 32-bit integer range `[-231, 231 - 1]`, then return
/// `0`. **Assume the environment does not allow you to store 64-bit integers (signed or
/// unsigned).** **Example 1:**
/// '''
/// **Input:** x = 123 **Output:** 321
/// '''
/// **Example 2:**
/// '''
/// **Input:** x = -123 **Output:** -321
/// '''
/// **Example 3:**
/// '''
/// **Input:** x = 120 **Output:** 21
/// '''
/// **Constraints:**
/// - `-231 <= x <= 231 - 1`
///
/// ## Examples
///
/// ### Example 1
///
///   123
///
/// ### Example 2
///
///   -123
///
/// ### Example 3
///
///   120
pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x < 10 && x > -10 {
            return x;
        }

        let mut ret: i64 = 0;
        let mut x = x;
        while x != 0 {
            ret = ret * 10 + x as i64 % 10;
            if ret > i32::MAX as i64 || ret < i32::MIN as i64 {
                return 0;
            }
            x /= 10;
        }

        ret as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
