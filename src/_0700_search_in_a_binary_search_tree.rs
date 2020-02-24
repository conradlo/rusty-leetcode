/*
 * @lc app=leetcode id=700 lang=rust
 *
 * [700] Search in a Binary Search Tree
 */

// @lc code=start
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
struct Solution;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(ref rc_node) = root {
            let node = rc_node.borrow();
            return match val.cmp(&node.val) {
                Ordering::Equal => Some(Rc::clone(&rc_node)),
                Ordering::Greater => Solution::search_bst(node.right.clone(), val),
                Ordering::Less => Solution::search_bst(node.left.clone(), val),
            };
        }
        None
    }
}
// @lc code=end
