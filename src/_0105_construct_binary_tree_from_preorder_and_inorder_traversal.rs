/*
 * @lc app=leetcode id=105 lang=rust
 *
 * [105] Construct Binary Tree from Preorder and Inorder Traversal
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
use std::rc::Rc;
impl Solution {
    // You may assume that duplicates do not exist in the tree.

    // 8ms 10.53%
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(&val) = preorder.get(0) {
            // root node
            let mut root = TreeNode::new(val);

            // split inorder at root's val
            let mut cursor = 0;
            for i in 0..inorder.len() {
                if inorder[i] == val {
                    cursor = i; // index of root's val in inorder
                }
            }
            let left_inorder = &inorder[0..cursor];
            let right_inorder = &inorder[cursor + 1..]; // cursor + 1, skip root

            let left_preorder = &preorder[1..left_inorder.len() + 1]; // start from 1, skip root's val
            let right_preorder = &preorder[(left_inorder.len() + 1)..];

            root.left = Solution::build_tree(left_preorder.to_vec(), left_inorder.to_vec());
            root.right = Solution::build_tree(right_preorder.to_vec(), right_inorder.to_vec());
            return Some(Rc::new(RefCell::new(root)));
        }
        None
    }
}
// @lc code=end

// cargo watch -x "test _0105_ -- --nocapture --test-threads=1"
#[cfg(test)]
mod tests {
    use super::*;

    pub fn postorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // copy from # 145
        let mut traverse = vec![];
        if let Some(rc_node) = root {
            let node = rc_node.borrow();
            traverse.append(&mut postorder_traversal_recursive(node.left.clone()));
            traverse.append(&mut postorder_traversal_recursive(node.right.clone()));
            traverse.push(node.val);
        }
        traverse
    }

    fn get_test_cases() -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> {
        // (preorder, inorder, postorder)
        return vec![
            (vec![], vec![], vec![]),
            (vec![1], vec![1], vec![1]),
            // [1, 2, 3]
            (vec![1, 2, 3], vec![2, 1, 3], vec![2, 3, 1]),
            // [1, null, 2, 3]
            (vec![1, 2, 3], vec![1, 3, 2], vec![3, 2, 1]),
            (
                // [3, 9, 20, null, null, 15, 7]
                vec![3, 9, 20, 15, 7],
                vec![9, 3, 15, 20, 7],
                vec![9, 15, 7, 20, 3],
            ),
            (
                // [5, 1, 4, null, null, 3, 6]
                vec![5, 1, 4, 3, 6],
                vec![1, 5, 3, 4, 6],
                vec![1, 3, 6, 4, 5],
            ),
            (
                // [5,4, 7, 3, null, 2, null, -1, null, 9]
                vec![5, 4, 3, -1, 7, 2, 9],
                vec![-1, 3, 4, 5, 9, 2, 7],
                vec![-1, 3, 4, 9, 2, 7, 5],
            ),
            (
                // [6,2,7,1,4,null,9,null,null,3,5,8]
                vec![6, 2, 1, 4, 3, 5, 7, 9, 8],
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 3, 5, 4, 2, 8, 9, 7, 6],
            ),
            (
                // [5,4,8,11,null,13,6,7,2,null,null,null,1]
                vec![5, 4, 11, 7, 2, 8, 13, 6, 1],
                vec![7, 11, 2, 4, 5, 13, 8, 6, 1],
                vec![7, 2, 11, 4, 13, 1, 6, 8, 5],
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (preorder, inorder, postorder) in testcases {
            // let _inorder = inorder.clone();
            // let _postorder = postorder.clone();
            let result = Solution::build_tree(preorder, inorder);
            // println!("{:?} {:?} | {:#?}", _inorder, _postorder, result);
            assert_eq!(postorder_traversal_recursive(result), postorder);
        }
    }
}
