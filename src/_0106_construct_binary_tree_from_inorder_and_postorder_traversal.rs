/*
 * @lc app=leetcode id=106 lang=rust
 *
 * [106] Construct Binary Tree from Inorder and Postorder Traversal
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
    // Given : You may assume that duplicates do not exist in the tree.

    // Observation
    // [1] in postorder, the last item (if any) must be the root of the whole tree

    // [2] in inorder, all value to the right of root's value (if any) must be belonged to right sub tree
    // -> we know the value set of the right sub tree
    // [3] in inorder, all value to the left of root's value (if any) must be belonged to left sub tree
    // -> we know the value set of the left sub tree

    // [4] in postorder, the first value next to the root's value and is a right sub tree values (in [2])
    // is the right child of root (if any)
    // [5] in postorder, the first value that are left sub tree value and closest the the end (root)
    // is the left child of root (if any)

    // if no left and right child, this node must be leaf
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut postorder = postorder;
        if let Some(node_value) = postorder.pop() {
            // create new node
            let mut node = TreeNode::new(node_value);

            // split inorder at node's value
            let mut split_inorder_iter = inorder.split(|val| *val == node_value);

            // get left & right inorder,postorder
            let left_sub_inorder = split_inorder_iter.next().unwrap();
            let left_sub_postorder = &postorder[..left_sub_inorder.len()];
            let right_sub_inorder = split_inorder_iter.next().unwrap();
            let right_sub_postorder = &postorder[left_sub_inorder.len()..postorder.len()];

            // recursive on left and right, with left and right inorder/postorder
            node.left =
                Solution::build_tree(left_sub_inorder.to_vec(), left_sub_postorder.to_vec());
            node.right =
                Solution::build_tree(right_sub_inorder.to_vec(), right_sub_postorder.to_vec());
            return Some(Rc::new(RefCell::new(node)));
        }
        None
    }
    // TODO: optimize speed and memory
    // iterative approach?
}
// @lc code=end

// cargo watch -x "test _0106_ -- --nocapture --test-threads=1"
#[cfg(test)]
mod tests {
    use super::*;

    pub fn preorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // copy from # 144
        let mut traverse = vec![];
        if let Some(node) = root {
            let node = node.borrow();
            traverse.push(node.val);
            traverse.append(&mut preorder_traversal_recursive(node.left.clone()));
            traverse.append(&mut preorder_traversal_recursive(node.right.clone()));
        }
        traverse
    }

    fn get_test_cases() -> Vec<(Vec<i32>, Vec<i32>, Vec<i32>)> {
        // (inorder, postorder, preorder)
        return vec![
            (vec![], vec![], vec![]),
            (vec![1], vec![1], vec![1]),
            // [1, 2, 3]
            (vec![2, 1, 3], vec![2, 3, 1], vec![1, 2, 3]),
            // [1, null, 2, 3]
            (vec![1, 3, 2], vec![3, 2, 1], vec![1, 2, 3]),
            (
                // [3, 9, 20, null, null, 15, 7]
                vec![9, 3, 15, 20, 7],
                vec![9, 15, 7, 20, 3],
                vec![3, 9, 20, 15, 7],
            ),
            (
                // [5, 1, 4, null, null, 3, 6]
                vec![1, 5, 3, 4, 6],
                vec![1, 3, 6, 4, 5],
                vec![5, 1, 4, 3, 6],
            ),
            (
                // [5,4, 7, 3, null, 2, null, -1, null, 9]
                vec![-1, 3, 4, 5, 9, 2, 7],
                vec![-1, 3, 4, 9, 2, 7, 5],
                vec![5, 4, 3, -1, 7, 2, 9],
            ),
            (
                // [6,2,7,1,4,null,9,null,null,3,5,8]
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
                vec![1, 3, 5, 4, 2, 8, 9, 7, 6],
                vec![6, 2, 1, 4, 3, 5, 7, 9, 8],
            ),
            (
                // [5,4,8,11,null,13,6,7,2,null,null,null,1]
                vec![7, 11, 2, 4, 5, 13, 8, 6, 1],
                vec![7, 2, 11, 4, 13, 1, 6, 8, 5],
                vec![5, 4, 11, 7, 2, 8, 13, 6, 1],
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (inorder, postorder, preorder) in testcases {
            // let _inorder = inorder.clone();
            // let _postorder = postorder.clone();
            let result = Solution::build_tree(inorder, postorder);
            // println!("{:?} {:?} | {:#?}", _inorder, _postorder, result);
            assert_eq!(preorder_traversal_recursive(result), preorder);
        }
    }
}
