#![allow(dead_code)]
#![allow(clippy::doc_lazy_continuation)]

/// Problem: Recover Binary Search Tree
/// Difficulty: Medium
/// URL: https://leetcode.com/problems/recover-binary-search-tree/
///
/// You are given the `root` of a binary search tree (BST), where the values of **exactly** two
/// nodes of the tree were swapped by mistake. *Recover the tree without changing its
/// structure*. **Example 1:**
/// '''
/// **Input:** root = [1,3,null,null,2] **Output:** [3,1,null,null,2] **Explanation:** 3 cannot
/// be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
/// '''
/// **Example 2:**
/// '''
/// **Input:** root = [3,1,4,null,null,2] **Output:** [2,1,4,null,null,3] **Explanation:** 2
/// cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
/// '''
/// **Constraints:**
/// - The number of nodes in the tree is in the range `[2, 1000]`.
/// - `-2^31 <= Node.val <= 2^31 - 1`
/// **Follow up:** A solution using `O(n)` space is pretty straight-forward. Could you devise a
///   constant `O(1)` space solution?
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
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first: Option<Rc<RefCell<TreeNode>>> = None;
        let mut second: Option<Rc<RefCell<TreeNode>>> = None;
        let mut prev: Option<Rc<RefCell<TreeNode>>> = None;

        // Morris traversal for O(1) space
        let mut current = root.clone();

        while let Some(node) = current {
            if node.borrow().left.is_none() {
                // Process current node
                Self::process_node(&node, &mut prev, &mut first, &mut second);
                current = node.borrow().right.clone();
            } else {
                // Find inorder predecessor
                let mut predecessor = node.borrow().left.clone();
                while predecessor.as_ref().unwrap().borrow().right.is_some()
                    && predecessor.as_ref().unwrap().borrow().right != Some(node.clone())
                {
                    let right = predecessor.as_ref().unwrap().borrow().right.clone();
                    predecessor = right;
                }

                if predecessor.as_ref().unwrap().borrow().right.is_none() {
                    // Make current the right child of predecessor
                    predecessor.as_ref().unwrap().borrow_mut().right = Some(node.clone());
                    current = node.borrow().left.clone();
                } else {
                    // Revert the changes
                    predecessor.as_ref().unwrap().borrow_mut().right = None;
                    // Process current node
                    Self::process_node(&node, &mut prev, &mut first, &mut second);
                    current = node.borrow().right.clone();
                }
            }
        }

        // Swap values of first and second
        if let (Some(f), Some(s)) = (first, second) {
            let temp = f.borrow().val;
            f.borrow_mut().val = s.borrow().val;
            s.borrow_mut().val = temp;
        }
    }

    fn process_node(
        current: &Rc<RefCell<TreeNode>>,
        prev: &mut Option<Rc<RefCell<TreeNode>>>,
        first: &mut Option<Rc<RefCell<TreeNode>>>,
        second: &mut Option<Rc<RefCell<TreeNode>>>,
    ) {
        if let Some(p) = prev
            && p.borrow().val > current.borrow().val
        {
            if first.is_none() {
                *first = Some(p.clone());
            }
            *second = Some(current.clone());
        }
        *prev = Some(current.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0099_1() {
        // Test with [1,3,null,null,2] -> should become [3,1,null,null,2]
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        })));

        Solution::recover_tree(&mut root);

        // Check that it's now valid: root should be 3, left should be 1
        assert_eq!(root.as_ref().unwrap().borrow().val, 3);
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            1
        );
    }

    #[test]
    fn test_case_0099_2() {
        // Test with [3,1,4,null,null,2] -> should become [2,1,4,null,null,3]
        let mut root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
        })));

        Solution::recover_tree(&mut root);

        // Check that it's now valid: root should be 2
        assert_eq!(root.as_ref().unwrap().borrow().val, 2);
        assert_eq!(
            root.as_ref()
                .unwrap()
                .borrow()
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .val,
            3
        );
    }
}
