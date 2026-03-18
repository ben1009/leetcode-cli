#![allow(dead_code)]

/// Problem: Reverse Integer
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/reverse-integer/
///
/// Given a signed 32-bit integer `x`, return `x`* with its digits reversed*. If reversing `x`
/// causes the value to go outside the signed 32-bit integer range `[-2^31, 2^31 - 1]`, then return
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
/// - `-2^31 <= x <= 2^31 - 1`
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

        let mut ret: i32 = 0;
        let mut x = x;
        while x != 0 {
            let Some(r) = ret.checked_mul(10).and_then(|r| r.checked_add(x % 10)) else {
                return 0;
            };
            ret = r;
            x /= 10;
        }

        ret
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0007() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
    }
}
