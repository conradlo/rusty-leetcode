/*
 * @lc app=leetcode id=779 lang=rust
 *
 * [779] K-th Symbol in Grammar
 */

struct Solution;
// @lc code=start
impl Solution {
    pub fn kth_grammar_v1(n: i32, k: i32) -> i32 {
        let mut list: Vec<i32> = vec![0];
        for _ in 1..n {
            let mut new: Vec<i32> = vec![];
            for el in list {
                if el == 0 {
                    new.push(0);
                    new.push(1);
                } else {
                    new.push(1);
                    new.push(0);
                }
            }
            list = new;
        }
        // println!("{} {:?}", n, list);
        list[k as usize - 1]
    }

    pub fn kth_grammar_v2(n: i32, k: i32) -> i32 {
        // walk down a conceptual binary tree from root

        // binary search by compare k with (number of leaf node) / 2
        // treat subtree's root as new root
        // change root's value depends on whether we go left/right
        // update k's value when go down the subtree

        // repeat until no more leaf nodes
        // i.e. reached leaf nodes of the original tree
        // return `root` (the val of leaf node at k)

        let mut root = 0;
        let mut leaf_pos = k;
        let mut no_of_leaf = 1 << (n - 1); // 2 ** (n - 1)

        // can still go down
        while no_of_leaf > 1 {
            // every level has 2 ** (l - 1) nodes
            // and half of them are on the left/right
            // because this is a perfect binary tree
            let half = no_of_leaf >> 1; // 2 ** (n - 2)
            if leaf_pos > half {
                // we go right
                root = if root == 0 { 1 } else { 0 };
                leaf_pos -= half; // <- why?
            }
            // go left do not need to change `root` or `k` (why?)

            // focus on the subtree
            no_of_leaf = half;
        }

        root
    }
}
// @lc code=end

// cargo watch -x "test _0779_ -- --test-threads=1 --nocapture"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_test_cases() -> Vec<(i32, i32, i32)> {
        // (N, K, Expect)
        vec![
            (1, 1, 0),
            (1, 1, 0),
            (2, 1, 0),
            (2, 2, 1),
            (4, 5, 1),
            (5, 15, 1),
            (5, 16, 0),
            // (20, 524_288, 1),
            // (30, 536_870_912, 1),
        ]
    }

    #[test]
    // #[ignore]
    fn test_v1() {
        for (n, k, expect) in get_test_cases() {
            let result = Solution::kth_grammar_v1(n, k);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    fn bench_v1(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::kth_grammar_v1(20, 524_288);
            assert_eq!(result, 1);
        });
    }

    #[test]
    fn test_v2() {
        for (n, k, expect) in get_test_cases() {
            let result = Solution::kth_grammar_v2(n, k);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_v2_2() {
        // logic of v1
        let mut list: Vec<i32> = vec![0];
        for _ in 1..20 {
            let mut new: Vec<i32> = vec![];
            for el in list {
                if el == 0 {
                    new.push(0);
                    new.push(1);
                } else {
                    new.push(1);
                    new.push(0);
                }
            }
            list = new;
        }
        // println!("{:?}", list);
        for k in 1..1 << (20 - 1) {
            let result = Solution::kth_grammar_v2(20, k);
            assert_eq!(result, list[k as usize - 1]);
        }
    }

    #[bench]
    fn bench_v2(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::kth_grammar_v2(30, 536_870_912);
            assert_eq!(result, 1);
        });
    }
}
