/*
 * @lc app=leetcode id=102 lang=rust
 *
 * [102] Binary Tree Level Order Traversal
 */

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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
// @lc code=start
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back((0, node)); // tag each node we visit with a "depth" number
        }
        while let Some((level, rc_node)) = q.pop_front() {
            let node = rc_node.borrow();
            // ans[0] = vector of node values at level 0 (i.e. root)
            // ans[1] = vector of node values at level 1 etc...
            // append node's value to corresponding vector
            // create new vector at ans[level] if it does not exist
            if let Some(v) = ans.get_mut(level) {
                v.push(node.val)
            } else {
                ans.push(vec![node.val]);
            };
            if let Some(ref rc_left) = node.left {
                q.push_back((level + 1, Rc::clone(&rc_left)));
            }
            if let Some(ref rc_right) = node.right {
                q.push_back((level + 1, Rc::clone(&rc_right)));
            }
        }
        ans
    }
}
// @lc code=end

// cargo watch -x "test _0102_ -- --nocapture --test-threads=1"
#[cfg(test)]
mod tests {
    use super::*;

    fn build_tree(list: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if list.len() < 1 {
            return None;
        }
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![None; list.len()];
        let mut cursor = 0; // !important
        for i in 0..list.len() {
            match list[i] {
                Some(k) => {
                    let tree_node = nodes[i]
                        .clone()
                        .unwrap_or(Rc::new(RefCell::new(TreeNode::new(k))));
                    if let Some(Some(ref left)) = list.get(cursor + 1) {
                        let left_node = Rc::new(RefCell::new(TreeNode::new(*left)));
                        nodes[cursor + 1] = Some(Rc::clone(&left_node));
                        tree_node.borrow_mut().left = Some(left_node);
                    }
                    if let Some(Some(ref right)) = list.get(cursor + 2) {
                        let right_node = Rc::new(RefCell::new(TreeNode::new(*right)));
                        nodes[cursor + 2] = Some(Rc::clone(&right_node));
                        tree_node.borrow_mut().right = Some(right_node);
                    }
                    nodes[i] = Some(tree_node);
                    cursor += 2;
                    if cursor > list.len() {
                        break;
                    }
                }
                None => (),
            }
        }
        nodes[0].clone()
    }

    fn get_test_cases() -> Vec<(Vec<Option<i32>>, Vec<Vec<i32>>)> {
        return vec![
            (vec![], vec![]),
            (vec![Some(1), Some(2), Some(3)], vec![vec![1], vec![2, 3]]),
            (
                vec![Some(1), None, Some(2), Some(3)],
                vec![vec![1], vec![2], vec![3]],
            ),
            (
                vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
                vec![vec![3], vec![9, 20], vec![15, 7]],
            ),
            (
                vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
                vec![vec![5], vec![1, 4], vec![3, 6]],
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
                vec![vec![5], vec![4, 7], vec![3, 2], vec![-1, 9]],
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
                vec![vec![6], vec![2, 7], vec![1, 4, 9], vec![3, 5, 8]],
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (tree_nodes, expect) in testcases {
            let tree = build_tree(&tree_nodes);
            let result = Solution::level_order(tree);
            assert_eq!(result, expect);
        }
    }
}
