/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */

use std::cmp::min;
use std::cmp::Ordering;
use std::collections::HashMap;
struct Solution;
// @lc code=start
impl Solution {
    // v1
    // DFS recursive memo table DP
    pub fn v1_helper(coins: &[i32], amount: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if let Some(cache) = memo.get(&amount) {
            return *cache;
        }
        match amount.cmp(&0) {
            Ordering::Less => std::i32::MAX,
            Ordering::Equal => 0,
            Ordering::Greater => {
                let mut ans = std::i32::MAX;
                for &coin in coins.iter() {
                    ans = min(ans, Solution::v1_helper(coins, amount - coin, memo))
                }
                ans = if ans < std::i32::MAX { 1 + ans } else { ans };
                memo.insert(amount, ans);
                ans
            }
        }
    }
    pub fn coin_change_v1(coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        let result = Solution::v1_helper(&coins, amount, &mut memo);
        if result < std::i32::MAX {
            result
        } else {
            -1
        }
    }

    // DP bottom up, "pull"
    pub fn coin_change_v2(coins: Vec<i32>, amount: i32) -> i32 {
        let amount = amount as usize;
        let mut dp: Vec<i32> = vec![std::i32::MAX; amount + 1];
        dp[0] = 0;
        for i in 1..=amount {
            let mut sub_ans = std::i32::MAX;
            for &coin in coins.iter() {
                let coin = coin as usize;
                if i >= coin {
                    sub_ans = min(sub_ans, dp[i - coin]);
                }
            }
            if sub_ans < std::i32::MAX {
                dp[i] = 1 + sub_ans;
            }
        }
        let ans = *dp.last().unwrap();
        if ans < std::i32::MAX {
            ans
        } else {
            -1
        }
    }
}
// @lc code=end

// cargo watch -x "test _0322_ -- --nocapture"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_test_cases() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![1, 2, 5], 11, 3),
            (vec![2], 3, -1),
            (vec![1, 3, 4], 6, 2),
            (vec![1, 3, 5], 12, 4),
            (vec![1, 2, 5, 10], 32, 4),
            (vec![2, 3, 5, 7], 101, 15),
            (vec![2, 4, 8], 101, -1),
            (vec![1, 2, 3], 0, 0),
            (vec![1, 2, 3, 5, 7, 11, 13, 17, 19], 1294, 69),
        ]
    }

    #[bench]
    fn run_test_cases_v1_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::coin_change_v1(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }

    #[bench]
    fn run_test_cases_v2_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::coin_change_v2(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }
}
