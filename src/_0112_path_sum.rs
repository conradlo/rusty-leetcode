/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
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
    // first intuition:
    // find all root-to-leaf sum, check if target exists

    // how to find root-to-leaf sum?
    // top-to-bottom recursive/iterative

    // TODO: recursive

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, sum: i32) -> bool {
        // @ref: # 114 binary tree preorder traversal
        // this is a binary tree preorder iterative algo
        let mut stack: Vec<(i32, Rc<RefCell<TreeNode>>)> = vec![];
        if let Some(ref rc_node) = root {
            stack.push((0, Rc::clone(&rc_node))); // tag node with accumulate sum
        }
        while let Some((acc, rc_node)) = stack.pop() {
            let node = rc_node.borrow();
            if node.left.is_none() && node.right.is_none() && node.val + acc == sum {
                // is leaf node, check sum
                return true;
            } else {
                if let Some(ref right_rc) = node.right {
                    stack.push((acc + node.val, Rc::clone(&right_rc))); // basically copy the acc sum to this path
                }
                if let Some(ref left_rc) = node.left {
                    stack.push((acc + node.val, Rc::clone(&left_rc))); // copy acc sum
                }
            }
        }
        false
    }
}
// @lc code=end

// cargo watch -x "test _0112_ -- --nocapture --test-threads=1"
#[cfg(test)]
mod tests {
    use super::*;

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
    type Input = Vec<Option<i32>>;
    type Expect = Vec<(i32, bool)>;

    fn get_test_cases() -> Vec<(Input, Expect)> {
        return vec![
            (vec![], vec![(0, false)]),
            (vec![None], vec![(0, false)]),
            (vec![Some(1)], vec![(1, true), (0, false)]),
            (
                vec![Some(1), Some(2), Some(3)],
                vec![(3, true), (4, true), (2, false)],
            ),
            (
                vec![Some(1), None, Some(2), Some(3)],
                vec![(6, true), (3, false)],
            ),
            (
                vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)],
                vec![(12, true), (38, true), (30, true), (100, false)],
            ),
            (
                vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
                vec![(6, true), (12, true), (15, true), (17, false)],
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
                vec![(11, true)],
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
                vec![
                    (0, false),
                    (9, true),
                    (15, true),
                    (17, true),
                    (30, true),
                    (31, false),
                ],
            ),
            (
                vec![
                    Some(5),
                    Some(4),
                    Some(8),
                    Some(11),
                    None,
                    Some(13),
                    Some(4),
                    Some(7),
                    Some(2),
                    None,
                    None,
                    None,
                    Some(1),
                ],
                vec![(27, true), (22, true), (26, true), (18, true), (0, false)],
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (tree_nodes, target_list) in testcases {
            let tree = build_tree(&tree_nodes);
            for (target_sum, expect) in target_list {
                let result = Solution::has_path_sum(tree.clone(), target_sum);
                // println!("{:?} {} | {}", tree_nodes, target_sum, result);
                assert_eq!(result, expect);
            }
        }
    }
}
