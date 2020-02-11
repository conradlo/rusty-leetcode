/*
 * @lc app=leetcode id=145 lang=rust
 *
 * [145] Binary Tree Postorder Traversal
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
    pub fn postorder_traversal_recursive(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut traverse = vec![];
        if let Some(rc_node) = root {
            let node = rc_node.borrow();
            traverse.append(&mut Solution::postorder_traversal_recursive(
                node.left.clone(),
            ));
            traverse.append(&mut Solution::postorder_traversal_recursive(
                node.right.clone(),
            ));
            traverse.push(node.val);
        }
        traverse
    }
    pub fn postorder_traversal_iterative_ver1(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // this version will modify the input binary tree
        // but easy to understand
        let mut ans = vec![];
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![];
        if let Some(node) = root {
            stack.push(node)
        }
        while let Some(top_node) = stack.pop() {
            let mut node = top_node.borrow_mut();
            if node.right.is_none() && node.left.is_none() {
                // if the top_node did not have child, process it
                ans.push(node.val);
            } else {
                // if the top_node has children
                // we detach the children from top_node
                // and then [1] push the top_node back to the stack
                // then [2] push right child (if any), then [3] push left child (if any)
                // such that the stack will become:
                /*
                   |left child | [3] (if any, process before right child)
                   |right child| [2] (if any, process before top_node)
                   |top_node   | [1] (now all children were detached)
                   |...        |
                */
                let right = node.right.clone();
                node.right = None; // detach
                let left = node.left.clone();
                node.left = None; // detach
                stack.push(top_node.clone());
                if let Some(right_node) = right {
                    stack.push(right_node);
                }
                if let Some(left_node) = left {
                    stack.push(left_node);
                }
            }
        }
        ans
    }

    // when to push/stop push, when to pop, when to process
    // if no extra logic to rmb what node we'd visited
    // everytime we pop, we must process (?)
    pub fn postorder_traversal_iterative_ver2(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // this version do not modify the input tree
        // we use a boolean flag to mark "visited" node
        // the order of process will still be left_child -> right_child -> parent
        // because we always [1] push parent node [2] push right_child node [3] push left_child node
        // and by stack's LIFO nature, we process the binary three's nodes in "postorder"
        let mut ans = vec![];
        let mut stack: Vec<(bool, Rc<RefCell<TreeNode>>)> = vec![];
        if let Some(node) = root {
            stack.push((false, node));
        }
        while let Some((examined, rc_node)) = stack.pop() {
            let node = rc_node.borrow(); // `node` is a Ref
            if examined {
                ans.push(node.val);
            } else {
                stack.push((true, Rc::clone(&rc_node)));
                if let Some(ref right_node) = node.right {
                    stack.push((false, Rc::clone(&right_node)));
                }
                if let Some(ref left_node) = node.left {
                    stack.push((false, Rc::clone(&left_node)));
                }
            }
        }
        ans
    }
}
// @lc code=end

// cargo watch -x "test _0145_ -- --nocapture --test-threads=1"
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

    fn get_test_cases() -> Vec<(Vec<Option<i32>>, Vec<i32>)> {
        return vec![
            (vec![], vec![]),
            (vec![Some(1), Some(2), Some(3)], vec![2, 3, 1]),
            (vec![Some(1), None, Some(2), Some(3)], vec![3, 2, 1]),
            (
                vec![Some(5), Some(1), Some(4), None, None, Some(3), Some(6)],
                vec![1, 3, 6, 4, 5],
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
                vec![-1, 3, 4, 9, 2, 7, 5],
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
                vec![1, 3, 5, 4, 2, 8, 9, 7, 6],
            ),
        ];
    }

    #[test]
    fn run_test_cases_1() {
        let testcases = get_test_cases();
        for (tree_nodes, expect) in testcases {
            let tree = build_tree(&tree_nodes);
            let result = Solution::postorder_traversal_recursive(tree);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn run_test_cases_2() {
        let testcases = get_test_cases();
        for (tree_nodes, _) in testcases {
            let tree = build_tree(&tree_nodes);
            let ans_recur = Solution::postorder_traversal_recursive(tree.clone());
            let ans_iter = Solution::postorder_traversal_iterative_ver1(tree.clone());
            assert_eq!(ans_recur, ans_iter);
        }
    }

    #[test]
    fn run_test_cases_3() {
        let testcases = get_test_cases();
        for (tree_nodes, _) in testcases {
            let tree = build_tree(&tree_nodes);
            let ans_recur = Solution::postorder_traversal_recursive(tree.clone());
            let ans_iter = Solution::postorder_traversal_iterative_ver2(tree.clone());
            assert_eq!(ans_recur, ans_iter);
        }
    }
}
