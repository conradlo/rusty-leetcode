/*
 * @lc app=leetcode id=144 lang=rust
 *
 * [144] Binary Tree Preorder Traversal
 */

#[allow(dead_code)]
struct Solution;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
//
#[allow(dead_code)]
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
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
#[allow(dead_code)]
impl Solution {
    pub fn preorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traverse = vec![];
        if let Some(node) = root {
            let ref node = node.borrow();
            traverse.push(node.val);
            traverse.append(&mut Solution::preorder_traversal_recursive(
                node.left.clone(),
            ));
            traverse.append(&mut Solution::preorder_traversal_recursive(
                node.right.clone(),
            ));
        }
        traverse
    }

    pub fn preorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        /*
           1. Do not re-visit node
           2. only process node's data once
        */
        let mut ans = vec![];
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![root];
        // ! pop on read
        while let Some(stack_node) = stack.pop() {
            if let Some(ref rc_node) = stack_node {
                let node = rc_node.borrow();
                ans.push(node.val);
                stack.push(node.right.clone()); // ! right first
                stack.push(node.left.clone())
            }
        }
        return ans;
    }
}
// @lc code=end
// cargo watch -x "test _0144_ -- --nocapture --test-threads=1"
#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(list: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let l = list.len();
        if l < 1 {
            return None;
        }
        // @ref: https://doc.rust-lang.org/book/ch15-05-interior-mutability.html#having-multiple-owners-of-mutable-data-by-combining-rct-and-refcellt
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        for val in list {
            let node = match val {
                Some(k) => Some(Rc::new(RefCell::new(TreeNode::new(*k)))),
                None => None,
            };
            nodes.push(node);
        }
        let mut root_node: Option<Rc<RefCell<TreeNode>>> = None;
        let mut cursor = 0; // !important
        for i in 0..l {
            if let Some(Some(node)) = nodes.get(i) {
                let mut tree_node = node.borrow_mut();
                if let Some(Some(sub_node)) = nodes.get(cursor + 1) {
                    let left_node = Rc::clone(sub_node);
                    tree_node.left = Some(left_node);
                }
                if let Some(Some(sub_node)) = nodes.get(cursor + 2) {
                    let right_node = Rc::clone(sub_node);
                    tree_node.right = Some(right_node);
                }
                if root_node == None {
                    root_node = Some(Rc::clone(&node));
                }
                cursor += 2;
                if cursor > l {
                    break;
                }
            }
        }
        root_node
    }

    #[test]
    #[ignore]
    fn test_build_tree() {
        let testcases: Vec<Vec<Option<i32>>> = vec![
            vec![],
            vec![Some(1), Some(2), Some(3)],
            vec![Some(1), None, Some(2), Some(3)],
            vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
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
        ];
        for (idx, list) in testcases.iter().enumerate() {
            let tree = build_tree(list);
            println!("{} {:#?}", idx, tree);
        }
    }

    fn get_test_cases() -> Vec<(Vec<Option<i32>>, Vec<i32>)> {
        return vec![
            (vec![], vec![]),
            (vec![Some(1), Some(2), Some(3)], vec![1, 2, 3]),
            (vec![Some(1), None, Some(2), Some(3)], vec![1, 2, 3]),
            (
                vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
                vec![5, 1, 4, 3, 6],
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
                vec![5, 4, 3, -1, 7, 2, 9],
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (tree_nodes, expect) in testcases {
            let tree = build_tree(&tree_nodes);
            let result = Solution::preorder_traversal_recursive(tree);
            assert_eq!(result, expect);
        }
    }
    #[test]
    fn run_test_cases_2() {
        let testcases = get_test_cases();
        for (tree_nodes, expect) in testcases {
            let tree = build_tree(&tree_nodes);
            let result = Solution::preorder_traversal_iterative(tree);
            assert_eq!(result, expect);
        }
    }
}
