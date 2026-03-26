#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

/// Problem: Minimum Number of Arrows to Burst Balloons
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/minimum-number-of-arrows-to-burst-balloons/
///
/// There are some spherical balloons taped onto a flat wall that represents the XY-plane. The
/// balloons are represented as a 2D integer array `points` where `points[i] = [x_start, x_end]`
/// denotes a balloon whose **horizontal diameter** stretches between `x_start` and `x_end`. You do
/// not know the exact y-coordinates of the balloons.
/// Arrows can be shot up **directly vertically** (in the positive y-direction) from different
/// points along the x-axis. A balloon with `x_start` and `x_end` is **burst** by an arrow shot at
/// `x` if `x_start <= x <= x_end`. There is **no limit** to the number of arrows that can be shot.
/// A shot arrow keeps traveling up infinitely, bursting any balloons in its path.
/// Given the array `points`, return *the **minimum** number of arrows that must be shot to burst
/// all balloons*.
/// **Example 1:**
/// '''
/// **Input:** points = [[10,16],[2,8],[1,6],[7,12]] **Output:** 2 **Explanation:** The balloons can
/// be burst by 2 arrows: - Shoot an arrow at x = 6, bursting the balloons [2,8] and [1,6]. - Shoot
/// an arrow at x = 11, bursting the balloons [10,16] and [7,12].
/// '''
/// **Example 2:**
/// '''
/// **Input:** points = [[1,2],[3,4],[5,6],[7,8]] **Output:** 4 **Explanation:** One arrow needs to
/// be shot for each balloon for a total of 4 arrows.
/// '''
/// **Example 3:**
/// '''
/// **Input:** points = [[1,2],[2,3],[3,4],[4,5]] **Output:** 2 **Explanation:** The balloons can be
/// burst by 2 arrows: - Shoot an arrow at x = 2, bursting the balloons [1,2] and [2,3]. - Shoot an
/// arrow at x = 4, bursting the balloons [3,4] and [4,5].
/// '''
/// **Constraints:**
/// - `1 <= points.length <= 10^5`
/// - `points[i].length == 2`
/// - `-2^31 <= x_start < x_end <= 2^31 - 1`
pub struct Solution;

impl Solution {
    // Solved by Kimi atomically
    pub fn find_min_arrow_shots(points: Vec<Vec<i32>>) -> i32 {
        if points.is_empty() {
            return 0;
        }

        let mut points = points;
        // Sort by end position
        points.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut arrows = 1;
        let mut current_end = points[0][1];

        for point in points.iter().skip(1) {
            if point[0] > current_end {
                // Need a new arrow
                arrows += 1;
                current_end = point[1];
            }
        }

        arrows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0452_1() {
        let points = vec![vec![10, 16], vec![2, 8], vec![1, 6], vec![7, 12]];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }

    #[test]
    fn test_case_0452_2() {
        let points = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8]];
        assert_eq!(Solution::find_min_arrow_shots(points), 4);
    }

    #[test]
    fn test_case_0452_3() {
        let points = vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]];
        assert_eq!(Solution::find_min_arrow_shots(points), 2);
    }
}
