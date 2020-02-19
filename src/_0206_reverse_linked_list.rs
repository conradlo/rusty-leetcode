/*
 * @lc app=leetcode id=206 lang=rust
 *
 * [206] Reverse Linked List
 */

struct Solution;
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
impl Solution {
    // recursive & iterative
    pub fn reverse_list_iterative(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut rev = None;
        let mut head = head;
        while let Some(mut node) = head {
            head = node.next; // this is a copy (I think)
            node.next = rev;
            rev = Some(node);
        }
        rev
    }

    pub fn recursive_helper(
        prev: Option<Box<ListNode>>,
        rev: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut rev = rev;
        if let Some(mut head) = prev {
            let tail = head.next;
            head.next = rev;
            rev = Some(head);
            return Solution::recursive_helper(tail, rev);
        }
        rev
    }

    // leetcode's recursive implementation is much more elegant, check it out!
    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        return Solution::recursive_helper(head, None);
    }
}
// @lc code=end

// cargo watch -x "test _0206_ -- --nocapture --test-threads=1"
// use rand;
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use test::Bencher;

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
    fn test_reverse_linked_list_iterative() {
        let testcases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2, 3], vec![3, 2, 1]),
            (vec![1, 2, 3, 4], vec![4, 3, 2, 1]),
            (vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
        ];
        for (input, expect) in testcases {
            let list = build_list(input);
            let result = Solution::reverse_list_iterative(list);
            let encode = encode_list(result);
            assert_eq!(encode, expect);
        }
    }

    #[test]
    fn test_reverse_linked_list_recursive() {
        let testcases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![], vec![]),
            (vec![1], vec![1]),
            (vec![1, 2, 3], vec![3, 2, 1]),
            (vec![1, 2, 3, 4], vec![4, 3, 2, 1]),
            (vec![1, 2, 3, 4, 5], vec![5, 4, 3, 2, 1]),
        ];
        for (input, expect) in testcases {
            let list = build_list(input);
            let result = Solution::reverse_list_recursive(list);
            let encode = encode_list(result);
            assert_eq!(encode, expect);
        }
    }

    #[test]
    fn test_random_input() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let n: usize = rng.gen_range(0, 50);
            // println!("{}", n);
            let mut rand_arr: Vec<i32> = vec![0; n];
            for i in 0..n {
                rand_arr[i] = rng.gen_range(1, 99);
            }
            let list = build_list(rand_arr);
            let ans_1 = Solution::reverse_list_iterative(list.clone());
            let ans_2 = Solution::reverse_list_recursive(list);
            // println!("{} : {}", ans_1, result);
            assert_eq!(encode_list(ans_1), encode_list(ans_2));
        }
    }

    #[bench]
    fn bench_reverse_list_iterative(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_arr: Vec<i32> = vec![0; 10000];
        for i in 0..rand_arr.len() {
            rand_arr[i] = rng.gen_range(1, 99);
        }
        let list = build_list(rand_arr);
        b.iter(|| {
            Solution::reverse_list_iterative(list.clone());
        })
    }

    #[bench]
    fn bench_reverse_list_recursive(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_arr: Vec<i32> = vec![0; 10000];
        for i in 0..rand_arr.len() {
            rand_arr[i] = rng.gen_range(1, 99);
        }
        let list = build_list(rand_arr);
        b.iter(|| {
            Solution::reverse_list_recursive(list.clone());
        })
    }
}
