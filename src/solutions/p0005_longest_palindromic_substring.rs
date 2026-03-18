#![allow(dead_code)]

/// Problem: Longest Palindromic Substring
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/longest-palindromic-substring/
///
/// Given a string `s`, return *the longest**palindromic**substring* in `s`.
/// **Example 1:**
/// '''
/// **Input:** s = "babad" **Output:** "bab" **Explanation:** "aba" is also a valid answer.
/// '''
/// **Example 2:**
/// '''
/// **Input:** s = "cbbd" **Output:** "bb"
/// '''
/// **Constraints:**
/// - `1 <= s.length <= 1000`
/// - `s` consist of only digits and English letters.
///
/// ## Examples
///
/// ### Example 1
///
///   "babad"
///
/// ### Example 2
///
///   "cbbd"
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }

        let mut ret: &[u8] = &[];
        let s = s.as_bytes();
        for i in 0..s.len() - 1 {
            let s1 = Solution::pali(s, i as i32, i);
            let s2 = Solution::pali(s, i as i32, i + 1);
            if s1.len() > ret.len() {
                ret = s1;
            }
            if s2.len() > ret.len() {
                ret = s2;
            }
        }

        String::from_utf8_lossy(ret).into_owned()
    }

    fn pali(s: &[u8], mut l: i32, mut r: usize) -> &[u8] {
        while l >= 0 && r < s.len() && s[l as usize] == s[r] {
            l -= 1;
            r += 1;
        }

        &s[(l + 1) as usize..r]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0005() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a");
    }
}
