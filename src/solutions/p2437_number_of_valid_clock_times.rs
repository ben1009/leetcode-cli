#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

/// Problem: Number of Valid Clock Times
/// Difficulty: Easy
/// URL: https://leetcode.com/problems/number-of-valid-clock-times/
///
/// You are given a string of length `5` called `time`, representing the current time on a digital
/// clock in the format `"hh:mm"`. The **earliest** possible time is `"00:00"` and the **latest**
/// possible time is `"23:59"`.
/// In the string `time`, the digits represented by the `?` symbol are **unknown**, and must be
/// **replaced** with a digit from `0` to `9`.
/// Return* an integer *`answer`*, the number of valid clock times that can be created by replacing
/// every *`?`* with a digit from *`0`* to *`9`.
/// **Example 1:**
/// '''
/// **Input:** time = "?5:00" **Output:** 2 **Explanation:** We can replace the ? with either a 0
/// or 1, producing "05:00" or "15:00". Note that we cannot replace it with a 2, since the time
/// "25:00" is invalid. In total, we have two choices.
/// '''
/// **Example 2:**
/// '''
/// **Input:** time = "0?:0?" **Output:** 100 **Explanation:** Each ? can be replaced by any digit
/// from 0 to 9, so we have 100 total choices.
/// '''
/// **Example 3:**
/// '''
/// **Input:** time = "??:??" **Output:** 1440 **Explanation:** There are 24 possible choices for
/// the hours, and 60 possible choices for the minutes. In total, we have 24 * 60 = 1440 choices.
/// '''
/// **Constraints:**
/// - `time` is a valid string of length `5` in the format `"hh:mm"`.
/// - `"00" <= hh <= "23"`
/// - `"00" <= mm <= "59"`
/// - Some of the digits might be replaced with `'?'` and need to be replaced with digits from `0`
///   to `9`.
pub struct Solution;

impl Solution {
    // Solved by Kimi atomically
    pub fn count_time(time: String) -> i32 {
        let chars: Vec<char> = time.chars().collect();
        let h1 = chars[0];
        let h2 = chars[1];
        let m1 = chars[3];
        let m2 = chars[4];

        let mut hour_count = 0;
        let mut minute_count = 0;

        // Count valid hours
        for h in 0..24 {
            let hh = format!("{:02}", h);
            let hh_chars: Vec<char> = hh.chars().collect();

            let valid = (h1 == '?' || h1 == hh_chars[0]) && (h2 == '?' || h2 == hh_chars[1]);
            if valid {
                hour_count += 1;
            }
        }

        // Count valid minutes
        for m in 0..60 {
            let mm = format!("{:02}", m);
            let mm_chars: Vec<char> = mm.chars().collect();

            let valid = (m1 == '?' || m1 == mm_chars[0]) && (m2 == '?' || m2 == mm_chars[1]);
            if valid {
                minute_count += 1;
            }
        }

        hour_count * minute_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_2528_1() {
        assert_eq!(Solution::count_time("?5:00".to_string()), 2);
    }

    #[test]
    fn test_case_2528_2() {
        assert_eq!(Solution::count_time("0?:0?".to_string()), 100);
    }

    #[test]
    fn test_case_2528_3() {
        assert_eq!(Solution::count_time("??:??".to_string()), 1440);
    }

    #[test]
    fn test_case_2528_4() {
        assert_eq!(Solution::count_time("12:00".to_string()), 1);
    }
}
