#![allow(dead_code)]

/// Problem: Add Two Numbers
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/add-two-numbers/
///
/// You are given two **non-empty** linked lists representing two non-negative integers. The digits
/// are stored in **reverse order**, and each of their nodes contains a single digit. Add the two
/// numbers and return the sum as a linked list. You may assume the two numbers do not contain any
/// leading zero, except the number 0 itself. **Example 1:**
/// '''
/// **Input:** l1 = [2,4,3], l2 = [5,6,4] **Output:** [7,0,8] **Explanation:** 342 + 465 = 807.
/// '''
/// **Example 2:**
/// '''
/// **Input:** l1 = [0], l2 = [0] **Output:** [0]
/// '''
/// **Example 3:**
/// '''
/// **Input:** l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9] **Output:** [8,9,9,9,0,0,0,1]
/// '''
/// **Constraints:**
/// - The number of nodes in each linked list is in the range `[1, 100]`.
/// - `0 <= Node.val <= 9`
/// - It is guaranteed that the list represents a number that does not have leading zeros.
///
/// ## Examples
///
/// ### Example 1
///
///   [2,4,3]
///
/// ### Example 2
///
///   [5,6,4]
///
/// ### Example 3
///
///   [0]
///
/// ### Example 4
///
///   [0]
///
/// ### Example 5
///
///   [9,9,9,9,9,9,9]
///
/// ### Example 6
///
///   [9,9,9,9]
pub struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut curr = &mut dummy;
        let mut l1 = l1;
        let mut l2 = l2;
        let mut carry = 0;

        while l1.is_some() || l2.is_some() || carry > 0 {
            let val1 = l1.as_ref().map(|n| n.val).unwrap_or(0);
            let val2 = l2.as_ref().map(|n| n.val).unwrap_or(0);
            let sum = val1 + val2 + carry;
            carry = sum / 10;
            curr.next = Some(Box::new(ListNode::new(sum % 10)));
            curr = curr.next.as_mut().unwrap();
            l1 = l1.and_then(|n| n.next);
            l2 = l2.and_then(|n| n.next);
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::p0002_add_two_numbers::{ListNode, Solution};

    #[test]
    fn test_case_0002() {
        // Input: [2,4,3]
        // Expected: [5,6,4]
        assert_eq!(
            Solution::add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 3, next: None }))
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode { val: 4, next: None }))
                    }))
                }))
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode { val: 8, next: None }))
                }))
            }))
        );
    }
}
