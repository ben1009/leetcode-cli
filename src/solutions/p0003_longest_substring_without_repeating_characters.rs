#![allow(dead_code)]

/// Problem: Longest Substring Without Repeating Characters
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/longest-substring-without-repeating-characters/
///
/// Given a string `s`, find the length of the **longest****substring** without duplicate
/// characters. **Example 1:**
/// '''
/// **Input:** s = "abcabcbb" **Output:** 3 **Explanation:** The answer is "abc", with the length of
/// 3. Note that "bca" and "cab" are also correct answers. '''
/// **Example 2:**
/// '''
/// **Input:** s = "bbbbb" **Output:** 1 **Explanation:** The answer is "b", with the length of 1.
/// '''
/// **Example 3:**
/// '''
/// **Input:** s = "pwwkew" **Output:** 3 **Explanation:** The answer is "wke", with the length of
/// 3. Notice that the answer must be a substring, "pwke" is a subsequence and not a substring. '''
/// **Constraints:**
/// - `0 <= s.length <= 5 * 104`
/// - `s` consists of English letters, digits, symbols and spaces.
///
/// ## Examples
///
/// ### Example 1
///
///   "abcabcbb"
///
/// ### Example 2
///
///   "bbbbb"
///
/// ### Example 3
///
///   "pwwkew"
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        if s.len() < 2 {
            return s.len() as i32;
        }

        let mut ret = 1;
        let mut pre = 0;
        let mut dic = HashMap::new();
        let s = s.as_bytes();
        for (i, item) in s.iter().enumerate() {
            if let Some(&k) = dic.get(item)
                && k >= pre
            {
                pre = k + 1;
            }
            dic.insert(item, i);
            ret = ret.max(i - pre + 1);
        }

        ret as i32
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0003() {
        // Input: "abcabcbb"
        // Expected: "bbbbb"
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }
}
