#![allow(dead_code)]

/// Problem: Palindrome Number
/// Difficulty: Easy
/// URL: https://leetcode.com/problems/palindrome-number/
///
/// Given an integer `x`, return `true`* if *`x`* is a ****palindrome****, and *`false`* otherwise*.
/// **Example 1:**
/// '''
/// **Input:** x = 121 **Output:** true **Explanation:** 121 reads as 121 from left to right and
/// from right to left. '''
/// **Example 2:**
/// '''
/// **Input:** x = -121 **Output:** false **Explanation:** From left to right, it reads -121. From
/// right to left, it becomes 121-. Therefore it is not a palindrome. '''
/// **Example 3:**
/// '''
/// **Input:** x = 10 **Output:** false **Explanation:** Reads 01 from right to left. Therefore it
/// is not a palindrome. '''
/// **Constraints:**
/// - `-2^31 <= x <= 2^31 - 1` **Follow up:** Could you solve it without converting the integer to a
///   string?
///
/// ## Examples
///
/// ### Example 1
///
///   121
///
/// ### Example 2
///
///   -121
///
/// ### Example 3
///
///   10
pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        if x % 10 == 0 {
            return false;
        }

        let mut ret: i64 = 0;
        let mut k = x;
        while k != 0 {
            let n = k as i64 % 10;
            ret = ret * 10 + n;
            k /= 10;
        }

        x as i64 == ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0009_1() {
        assert!(Solution::is_palindrome(121));
        assert!(!Solution::is_palindrome(-121));
        assert!(!Solution::is_palindrome(10));
        assert!(Solution::is_palindrome(1001));
    }
}
