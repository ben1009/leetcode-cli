#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

/// Problem: Check Completeness of a Binary Tree
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/check-completeness-of-a-binary-tree/
///
/// Given the `root` of a binary tree, determine if it is a *complete binary tree*.
/// In a **[complete binary
/// tree](http://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees)**, every level, except
/// possibly the last, is completely filled, and all nodes in the last level are as far left as
/// possible. It can have between `1` and `2^h` nodes inclusive at the last level `h`.
/// **Example 1:**
/// '''
/// **Input:** root = [1,2,3,4,5,6] **Output:** true **Explanation:** Every level before the
/// last is full (ie. levels with node-values {1} and {2, 3}), and all nodes in the last level
/// ({4, 5, 6}) are as far left as possible.
/// '''
/// **Example 2:**
/// '''
/// **Input:** root = [1,2,3,4,5,null,7] **Output:** false **Explanation:** The node with value
/// 7 isn't as far left as possible.
/// '''
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[1, 100]`.
/// - `1 <= Node.val <= 1000`
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    // Solved by Kimi atomically
    pub fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let mut queue = std::collections::VecDeque::new();
        queue.push_back(root.clone());

        let mut found_null = false;

        while let Some(node_opt) = queue.pop_front() {
            if let Some(node) = node_opt {
                if found_null {
                    return false;
                }
                let node = node.borrow();
                queue.push_back(node.left.clone());
                queue.push_back(node.right.clone());
            } else {
                found_null = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0958_1() {
        // Test with [1,2,3,4,5,6]
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                right: None,
            }))),
        })));
        assert!(Solution::is_complete_tree(root));
    }

    #[test]
    fn test_case_0958_2() {
        // Test with [1,2,3,4,5,null,7] - not complete
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));
        assert!(!Solution::is_complete_tree(root));
    }
}
