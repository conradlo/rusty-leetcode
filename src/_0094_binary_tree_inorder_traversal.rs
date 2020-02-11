/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
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
// @lc code=start
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn inorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traverse = vec![];
        if let Some(node) = root {
            let tree_node = node.borrow();
            traverse.append(&mut Solution::inorder_traversal_recursive(
                tree_node.left.clone(),
            ));
            traverse.push(tree_node.val);
            traverse.append(&mut Solution::inorder_traversal_recursive(
                tree_node.right.clone(),
            ));
        }
        traverse
    }

    // TODO: revisit
    pub fn inorder_traversal_iterative(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut current = root;
        while current.is_some() || stack.len() > 0 {
            if let Some(node) = current {
                // push until no more left child
                stack.push(Rc::clone(&node));
                current = node.borrow().left.clone();
            } else if let Some(top_node) = stack.pop() {
                // pop top_node and never come back
                ans.push(top_node.borrow().val);
                current = top_node.borrow().right.clone();
            }
        }
        ans
    }

    pub fn inorder_traversal_iterative_1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans: Vec<i32> = vec![];
        // let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        let mut stack: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        let mut current: Option<Rc<RefCell<TreeNode>>> = root;
        while current.is_some() || stack.len() > 0 {
            while let Some(ref node) = current.clone() {
                stack.push(current.clone());
                current = node.borrow().left.clone();
            }
            if let Some(Some(top)) = stack.pop() {
                ans.push(top.borrow().val);
                current = top.borrow().right.clone();
            }
        }
        ans
    }
}
// @lc code=end

// cargo watch -x "test _0094_ -- --nocapture --test-threads=1"
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

    fn build_tree_prev(list: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        // copy from _0144_
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
        ];
        for (_, list) in testcases.iter().enumerate() {
            let tree_prev = build_tree_prev(list);
            let tree = build_tree(list);
            // println!("{} {:#?}", idx, tree);
            assert_eq!(tree_prev, tree);
        }
    }

    fn get_test_cases() -> Vec<(Vec<Option<i32>>, Vec<i32>)> {
        return vec![
            (vec![], vec![]),
            (vec![Some(1), Some(2), Some(3)], vec![2, 1, 3]),
            (vec![Some(1), None, Some(2), Some(3)], vec![1, 3, 2]),
            (
                vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
                vec![1, 5, 3, 4, 6],
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
                vec![-1, 3, 4, 5, 9, 2, 7],
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
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (tree_nodes, expect) in testcases {
            let tree = build_tree(&tree_nodes);
            let result = Solution::inorder_traversal_recursive(tree);
            assert_eq!(result, expect);
        }
    }
    #[test]
    fn run_test_cases_2() {
        let testcases = get_test_cases();
        for (tree_nodes, expect) in testcases {
            let tree = build_tree(&tree_nodes);
            let result = Solution::inorder_traversal_iterative(tree);
            assert_eq!(result, expect);
        }
    }
}
