/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

// @lc code=start
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
struct Solution;
impl Solution {
    // time: Theta(|m| + |n|)
    // space: Theta(|m| + |n|)
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        // implicit return
        match (l1, l2) {
            (Some(l_node), Some(r_node)) => {
                if l_node.val <= r_node.val {
                    let mut hd = ListNode::new(l_node.val);
                    hd.next = Solution::merge_two_lists(l_node.next, Some(r_node));
                    Some(Box::new(hd))
                } else {
                    let mut hd = ListNode::new(r_node.val);
                    hd.next = Solution::merge_two_lists(Some(l_node), r_node.next);
                    Some(Box::new(hd))
                }
            }
            (Some(l_node), None) => Some(l_node),
            (None, Some(r_node)) => Some(r_node),
            (None, None) => None,
        }
    }

    // can do it iteratively?
}
// @lc code=end

// cargo watch -x "test _0021_ -- --nocapture --test-threads=1"
#[cfg(test)]
mod tests {
    use super::*;

    // helper for ListNode

    fn build_list(v: &[i32]) -> Option<Box<ListNode>> {
        let mut list: Option<Box<ListNode>> = None;
        for &val in v.iter().rev() {
            let mut new_node = Box::new(ListNode::new(val));
            new_node.next = list;
            list = Some(new_node);
        }
        list
    }

    #[test]
    fn test_build_list() {
        let testcases: Vec<(Vec<i32>, Option<Box<ListNode>>)> = vec![
            (vec![], None),
            (vec![1], Some(Box::new(ListNode::new(1)))),
            (
                vec![1, 2, 3],
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            ),
        ];
        for (input, expect) in testcases {
            let result = build_list(&input);
            assert_eq!(result, expect);
        }
    }

    fn encode_list(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut encoded = vec![];
        let mut hd = list;
        while let Some(node) = hd {
            encoded.push(node.val);
            hd = node.next;
        }
        encoded
    }

    #[test]
    fn test_encode_list() {
        let testcases: Vec<(Option<Box<ListNode>>, Vec<i32>)> = vec![
            (None, vec![]),
            (Some(Box::new(ListNode::new(1))), vec![1]),
            (
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
                vec![1, 2, 3],
            ),
        ];
        for (input, expect) in testcases {
            let result = encode_list(input);
            assert_eq!(result, expect);
        }
    }

    type Input = (Vec<i32>, Vec<i32>);
    type Expect = Vec<i32>;

    #[test]
    fn test_merge_two_sorted_lists() {
        let testcases: Vec<(Input, Expect)> = vec![
            ((vec![], vec![]), vec![]),
            ((vec![1], vec![]), vec![1]),
            ((vec![], vec![1]), vec![1]),
            ((vec![3, 4], vec![1, 2]), vec![1, 2, 3, 4]),
            ((vec![1, 2], vec![2, 3]), vec![1, 2, 2, 3]),
            ((vec![1, 2, 4], vec![1, 3, 4]), vec![1, 1, 2, 3, 4, 4]),
            (
                (vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10]),
                vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
            ),
        ];
        for ((left, right), expect) in testcases {
            let left_list = build_list(&left);
            let right_list = build_list(&right);

            let ans = Solution::merge_two_lists(left_list, right_list);

            assert_eq!(encode_list(ans), expect);
        }
    }
}
