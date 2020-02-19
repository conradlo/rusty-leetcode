/*
 * @lc app=leetcode id=24 lang=rust
 *
 * [24] Swap Nodes in Pairs
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
    // recursion
    // why would I think of using recursion from the first place?
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut fst) = head {
            if let Some(mut snd) = fst.next {
                fst.next = Solution::swap_pairs(snd.next);
                snd.next = Some(fst);
                return Some(snd);
            } else {
                return Some(fst);
            }
        }
        None
    }
}
// @lc code=end

// cargo watch -x "test _0024_"
#[cfg(test)]
mod tests {
    use super::*;

    fn build_list(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut hd: Option<Box<ListNode>> = None;
        for i in (0..v.len()).rev() {
            if hd.is_none() {
                hd = Some(Box::new(ListNode::new(v[i])));
            } else {
                let mut node = ListNode::new(v[i]);
                node.next = hd;
                hd = Some(Box::new(node));
            }
        }
        hd
    }

    #[test]
    fn test_build_list() {
        let testcases: Vec<(Vec<i32>, Option<Box<ListNode>>)> = vec![
            (vec![], None),
            (vec![1], Some(Box::new(ListNode { val: 1, next: None }))),
            (
                vec![1, 2, 3, 4],
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode { val: 4, next: None })),
                        })),
                    })),
                })),
            ),
        ];
        for (input, expect) in testcases {
            let result = build_list(input);
            assert_eq!(result, expect);
        }
    }

    fn encode_list(l: Option<Box<ListNode>>) -> Vec<i32> {
        let mut encode = vec![];
        let mut head = l;
        while let Some(node) = head {
            encode.push(node.val);
            head = node.next;
        }
        encode
    }

    #[test]
    fn test_encode_list() {
        let testcases: Vec<(Option<Box<ListNode>>, Vec<i32>)> = vec![
            (None, vec![]),
            (Some(Box::new(ListNode { val: 1, next: None })), vec![1]),
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

    #[test]
    fn test_swap_pairs() {
        let testcases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2, 3], vec![2, 1, 3]),
            (vec![1, 2, 3, 4], vec![2, 1, 4, 3]),
            (vec![1, 2, 3, 4, 5], vec![2, 1, 4, 3, 5]),
        ];
        for (input, expect) in testcases {
            let list = build_list(input);
            let result = Solution::swap_pairs(list);
            let encode = encode_list(result);
            assert_eq!(encode, expect);
        }
    }
}
