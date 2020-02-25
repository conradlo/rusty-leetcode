/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
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
/**
 * Depth First/ Breadth First
 * bottom up(~post order)/ top down (~pre order)
 * iterative/recursive
 */
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth_recursive_v1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // bottom up recursive
        // this is the most intuitive flow (for me)
        if let Some(rc_node) = root {
            let node = rc_node.borrow();
            return 1 + std::cmp::max(
                Solution::max_depth_recursive_v1(node.right.clone()),
                Solution::max_depth_recursive_v1(node.left.clone()),
            );
        }
        0
    }
    pub fn max_depth_recursive_v2_helper(node: Rc<RefCell<TreeNode>>) -> i32 {
        let node = node.borrow();
        let right_side_depth = if let Some(ref right_node) = node.right {
            Solution::max_depth_recursive_v2_helper(Rc::clone(&right_node)) // avoid cloning whole sub tree
        } else {
            0
        };
        let left_side_depth = if let Some(ref left_node) = node.left {
            Solution::max_depth_recursive_v2_helper(Rc::clone(&left_node)) // avoid cloning whole sub tree
        } else {
            0
        };
        // avoid using std::cmp:max
        1 + if left_side_depth > right_side_depth {
            left_side_depth
        } else {
            right_side_depth
        }
    }
    pub fn max_depth_recursive_v2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // this is an improved(?) version of recursive v1
        if let Some(rc_node) = root {
            return Solution::max_depth_recursive_v2_helper(rc_node);
        }
        0
    }

    pub fn max_depth_iterative_v1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // DFS preorder iterative
        let mut depth = 0;
        let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> = vec![];
        if let Some(rc_node) = root {
            stack.push((1, rc_node));
        }

        while let Some((level, rc_node)) = stack.pop() {
            let node = rc_node.borrow();
            if let Some(ref rc_right) = node.right {
                stack.push((level + 1, Rc::clone(&rc_right)));
            }
            if let Some(ref rc_left) = node.left {
                stack.push((level + 1, Rc::clone(&rc_left)));
            }
            if node.right.is_none() && node.left.is_none() && level > depth {
                depth = level;
            }
        }

        depth
    }
}
// @lc code=end

// cargo watch -x "test _0104_ -- --nocapture --test-threads=1"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn build_tree(list: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if list.is_empty() {
            return None;
        }
        let mut nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![None; list.len()];
        let mut cursor = 0; // !important
        for i in 0..list.len() {
            if let Some(k) = list[i] {
                let tree_node = nodes[i]
                    .clone()
                    .unwrap_or_else(|| Rc::new(RefCell::new(TreeNode::new(k))));
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
        }
        nodes[0].clone()
    }

    fn get_test_cases() -> Vec<(Vec<Option<i32>>, i32)> {
        return vec![
            (vec![], 0),
            (vec![None], 0),
            (vec![Some(1)], 1),
            (vec![Some(1), Some(2), Some(3)], 2),
            (vec![Some(1), None, Some(2), Some(3)], 3),
            (
                vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
                3,
            ),
            (
                vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
                3,
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
                4,
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
                4,
            ),
        ];
    }

    #[bench]
    fn test_and_bench_recursive_v1(b: &mut Bencher) {
        let testcases = get_test_cases();
        b.iter(|| {
            for (tree_nodes, expect) in testcases.clone() {
                let tree = build_tree(&tree_nodes);
                let result = Solution::max_depth_recursive_v1(tree);
                assert_eq!(result, expect);
            }
        });
    }

    #[bench]
    fn test_and_bench_recursive_v2(b: &mut Bencher) {
        let testcases = get_test_cases();
        b.iter(|| {
            for (tree_nodes, expect) in testcases.clone() {
                let tree = build_tree(&tree_nodes);
                let result = Solution::max_depth_recursive_v2(tree);
                assert_eq!(result, expect);
            }
        });
    }

    #[bench]
    fn test_and_bench_iterative_v1(b: &mut Bencher) {
        let testcases = get_test_cases();
        b.iter(|| {
            for (tree_nodes, expect) in testcases.clone() {
                let tree = build_tree(&tree_nodes);
                let result = Solution::max_depth_iterative_v1(tree);
                assert_eq!(result, expect);
            }
        });
    }
}
