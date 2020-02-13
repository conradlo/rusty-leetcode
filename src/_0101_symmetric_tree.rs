/*
 * @lc app=leetcode id=101 lang=rust
 *
 * [101] Symmetric Tree
 */

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
// At first I think we can "flatten" the tree into an array of values
// with inorder traversal, then check for symmetry
// but this approach is wrong, some asymmetric tree can gives symmetric array of values (see test cases)
//
// then I think we can do a BFS on the tree, and check for symmetry at each level of the tree
// at each level, there should be (2 ^ level) number of nodes (with root's level = 0)
// then the (2 ^ level) / 2 nodes on the LHS and (2 ^ level) / 2 nodes on the RHS should be
// in reverse order with each other (can check with a stack or reverse array value and compare etc)
// but then realize in practice number of nodes at each level could be small than (2 ^ level), when upper levels have leaf child of None

// Frankly, this question is beyond "Easy" to me

// @lc code=start
use std::cell::RefCell;
// use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn inorder(opt_node: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(rc_node) = opt_node {
            let node = rc_node.borrow();
            let mut traverse = vec![];
            traverse.append(&mut Solution::inorder(node.left.clone()));
            traverse.push(node.val);
            traverse.append(&mut Solution::inorder(node.right.clone()));
            return traverse;
        }
        vec![]
    }
    pub fn wrong_answer_is_symmetric_dfs(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let inorder = Solution::inorder(root);
        let length = inorder.len();
        // println!("{:?}", inorder);
        if length < 2 {
            return true;
        } else if length % 2 == 0 {
            return false;
        } else {
            let m = (length - 1) >> 1;
            let left = &inorder[0..m];
            let right = &inorder[m + 1..length];
            for i in 0..m {
                if left[i] != right[m - i - 1] {
                    return false;
                }
            }
        }
        true // root is None
    }

    // this is the standard answer (recursive)
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(rc_node) = root {
            let node = rc_node.borrow();
            return Solution::is_mirror(node.left.clone(), node.right.clone());
        }
        true // root is None
    }
    pub fn is_mirror(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        if left.is_some() && right.is_some() {
            let left_rc = left.unwrap();
            let left_node = left_rc.borrow();
            let right_rc = right.unwrap();
            let right_node = right_rc.borrow();
            return left_node.val == right_node.val
                && Solution::is_mirror(left_node.left.clone(), right_node.right.clone())
                && Solution::is_mirror(right_node.left.clone(), left_node.right.clone());
        }

        false
    }
}
// @lc code=end

// cargo watch -x "test _0101_ -- --nocapture --test-threads=1"
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

    fn get_test_cases() -> Vec<(Vec<Option<i32>>, bool)> {
        return vec![
            (vec![], true),
            (vec![None], true),
            (vec![Some(1)], true),
            (vec![Some(1), None, Some(3)], false),
            (vec![Some(1), Some(2), None], false),
            (vec![Some(1), Some(2), Some(2)], true),
            (vec![Some(1), Some(2), Some(3)], false),
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
                false,
            ),
            (
                vec![
                    Some(1),
                    Some(2),
                    Some(2),
                    Some(3),
                    Some(4),
                    Some(4),
                    Some(3),
                ],
                true,
            ),
            (
                vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)],
                false,
            ),
            (
                vec![Some(1), Some(2), Some(2), None, Some(2), None, Some(2)],
                false,
            ),
            (
                vec![Some(1), Some(2), Some(2), None, Some(3), Some(3), None],
                true,
            ),
            (
                vec![
                    Some(1),
                    Some(2),
                    Some(2),
                    None,
                    Some(3),
                    Some(3),
                    None,
                    Some(4),
                    None,
                    None,
                    Some(4),
                ],
                true,
            ),
            (
                vec![Some(1), Some(2), Some(2), Some(2), None, Some(2)],
                false,
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (tree_nodes, expect) in testcases {
            let tree = build_tree(&tree_nodes);
            let result = Solution::is_symmetric(tree);
            assert_eq!(result, expect);
        }
    }
}
