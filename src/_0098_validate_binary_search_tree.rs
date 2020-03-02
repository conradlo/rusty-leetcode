/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
 */

// Definition for a binary tree node.
use crate::other::helper::TreeNode;

struct Solution;
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    // how to validate a binary search tree?

    // 1. use bst's sorted property

    // walk thru the tree "inorder"
    // see if all values of the tree is sorted
    // the bst is in ascending order, stated in the question
    // beware of the values' bound, i.e. the i32::MIN case
    //
    // Time: O(n) visit all nodes once in worst case
    // Space: max size of the stack, O(lg n) if tree is balanced, O(n) in worst case
    pub fn is_valid_bst_v1(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // iterative inorder traversal of a binary tree
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut current: Option<Rc<RefCell<TreeNode>>> = root;
        // rmb last visited value in the tree
        let mut last_val: Option<i32> = None;

        while current.is_some() || !stack.is_empty() {
            if let Some(rc_node) = current {
                let node = rc_node.borrow();
                stack.push(Rc::clone(&rc_node));
                current = node.left.clone();
            } else if let Some(rc_node) = stack.pop() {
                let node = rc_node.borrow();
                // check if values are ascending
                // println!("{}", node.val);
                if let Some(last) = last_val {
                    if last >= node.val {
                        // can return immediately
                        return false;
                    }
                }
                last_val = Some(node.val);
                current = node.right.clone();
            }
        }
        // walked thru whole tree
        true
    }

    // 2. use bst's recursive definition
    // do not just check the value of left & right node
    // need to check for left's max and right's min
    // Time: O(n) DFS whole tree
    // Space: max size of call stack, max O(n), O(lg n) for balanced tree
    pub fn check_bst_node_recursive(
        node: Option<Rc<RefCell<TreeNode>>>,
    ) -> (Option<i32>, bool, Option<i32>) {
        if let Some(rc_node) = node {
            let node = rc_node.borrow();

            let (left_min, is_valid_left_subtree, left_max) =
                Solution::check_bst_node_recursive(node.left.clone());
            let (right_min, is_valid_right_subtree, right_max) =
                Solution::check_bst_node_recursive(node.right.clone());

            let is_valid = match (left_max, right_min) {
                (Some(left_max_val), None) => left_max_val < node.val,
                (None, Some(right_min_val)) => node.val < right_min_val,
                (Some(left_max_val), Some(right_min_val)) => {
                    left_max_val < node.val && node.val < right_min_val
                }
                (None, None) => true,
            };
            let min_val = if let Some(left_min_val) = left_min {
                if node.val < left_min_val {
                    node.val
                } else {
                    left_min_val
                }
            } else {
                node.val
            };
            let max_val = if let Some(right_max_val) = right_max {
                if right_max_val < node.val {
                    node.val
                } else {
                    right_max_val
                }
            } else {
                node.val
            };
            (
                Some(min_val),
                is_valid && is_valid_left_subtree && is_valid_right_subtree,
                Some(max_val),
            )
        } else {
            (None, true, None)
        }
    }
    pub fn is_valid_bst_v2(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let (_, is_valid, _) = Solution::check_bst_node_recursive(root);
        is_valid
    }
}
// @lc code=end

// cargo watch -x "test _0098_ -- test-threads=1 --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    use crate::other::helper::build_tree;

    fn get_test_cases() -> Vec<(Vec<Option<i32>>, bool)> {
        vec![
            (vec![], true),
            (vec![Some(1)], true),
            (vec![Some(1), Some(1), Some(1)], false),
            (vec![Some(2), Some(1), Some(3)], true),
            (vec![Some(1), None, Some(2), Some(3)], false),
            (
                vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
                false,
            ),
            (
                vec![
                    Some(5),
                    Some(4),
                    Some(7),
                    Some(3),
                    None,
                    Some(2),
                    None,
                    Some(-1),
                    None,
                    Some(9),
                ],
                false,
            ),
            (
                vec![
                    Some(6),
                    Some(2),
                    Some(7),
                    Some(1),
                    Some(4),
                    None,
                    Some(9),
                    None,
                    None,
                    Some(3),
                    Some(5),
                    Some(8),
                ],
                true,
            ),
            (vec![Some(-2_147_483_648)], true),
            (
                vec![
                    Some(-2_147_483_648),
                    Some(-2_147_483_648),
                    Some(-2_147_483_648),
                ],
                false,
            ),
            (vec![Some(1), Some(-2_147_483_648), None], true),
            (vec![Some(2_147_483_647)], true),
            (
                vec![
                    Some(2_147_483_647),
                    Some(2_147_483_647),
                    Some(2_147_483_647),
                ],
                false,
            ),
            (vec![Some(1), None, Some(2_147_483_647)], true),
            (vec![Some(2_147_483_647), None, Some(1)], false),
            (
                vec![
                    Some(2_147_483_646),
                    Some(2_147_483_645),
                    Some(2_147_483_647),
                ],
                true,
            ),
            (
                vec![Some(10), Some(5), Some(15), None, None, Some(6), Some(20)],
                false,
            ),
        ]
    }

    #[test]
    fn test_is_valid_bst_v1() {
        for (list, expect) in get_test_cases() {
            let tree = build_tree(&list);
            let result = Solution::is_valid_bst_v1(tree);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_is_valid_bst_v2() {
        for (list, expect) in get_test_cases() {
            // println!("{:?}", list);
            let tree = build_tree(&list);
            let result = Solution::is_valid_bst_v2(tree);
            assert_eq!(result, expect);
        }
    }
}
