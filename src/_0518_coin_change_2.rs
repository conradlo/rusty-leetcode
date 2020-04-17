/*
 * @lc app=leetcode id=518 lang=rust
 *
 * [518] Coin Change 2
 */

struct Solution;
use std::cmp::Ordering;
use std::collections::HashMap;
// @lc code=start
impl Solution {
    // recursive DFS
    // shrink available coins along the way to avoid duplicated combinations
    // 36 ms
    pub fn v1_helper(
        amount: i32,
        coins: &[i32],
        last_index: usize,
        memo: &mut HashMap<(i32, usize), i32>,
    ) -> i32 {
        if let Some(cache) = memo.get(&(amount, last_index)) {
            return *cache;
        }
        match amount.cmp(&0) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => {
                let mut ans = 0;
                for i in last_index..coins.len() {
                    ans += Solution::v1_helper(amount - coins[i], &coins, i, memo);
                }
                memo.insert((amount, last_index), ans);
                ans
            }
        }
    }
    pub fn change_v1(amount: i32, coins: Vec<i32>) -> i32 {
        let mut coins = coins;
        coins.sort_by(|a, b| b.cmp(&a));
        let mut memo = HashMap::new();
        let ans = Solution::v1_helper(amount, &coins, 0, &mut memo);
        // debug
        // help derive the dp table
        // println!("{:?}", memo);
        // println!("==== amount: {:?} | coins: {:?}", amount, coins);
        // let mut table: Vec<Vec<i32>> = vec![vec![-1; coins.len()]; amount as usize + 1];
        // // memo.sort
        // for (&(subamount, coin_index), &count) in memo.iter() {
        //     table[subamount as usize][coin_index] = count;
        // }
        // for (i, row) in table.iter().enumerate().rev() {
        //     let entry = row.iter().rev().fold(String::from(""), |result, item| {
        //         result + &format!("{:5}", item)
        //     });
        //     println!("{: >3}| {}", i, entry);
        // }
        ans
    }

    // dp table bottom up
    // 8 ms
    // assume coins are sorted from small to large? NO
    pub fn change_v2(amount: i32, coins: Vec<i32>) -> i32 {
        let amount = amount as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; coins.len() + 1]; amount + 1];
        dp[0] = vec![1; coins.len() + 1]; // padding
        for subamount in 1..=amount {
            for coin_i in 1..=coins.len() {
                dp[subamount][coin_i] = dp[subamount][coin_i - 1]
                    + if subamount >= coins[coin_i - 1] as usize {
                        dp[subamount - coins[coin_i - 1] as usize][coin_i]
                    } else {
                        0
                    };
            }
        }
        *dp[amount].last().unwrap()
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
            (vec![1, 2, 5], 5, 4),
            (vec![2], 3, 0),
            (vec![10], 10, 1),
            (vec![1, 2, 3, 5, 7, 9], 101, 85842),
            (vec![1], 0, 1),
            (vec![1, 2, 5, 101], 500, 27901),
            (vec![2, 5, 1], 5, 4),
        ]
    }

    #[bench]
    fn run_test_cases_v1_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (coins, amount, expect) in testcases {
            b.iter(|| {
                let result = Solution::change_v1(amount, coins.clone());
                assert_eq!(result, expect);
            });
        }
    }
    #[bench]
    fn run_test_cases_v2_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (coins, amount, expect) in testcases {
            b.iter(|| {
                let result = Solution::change_v2(amount, coins.clone());
                assert_eq!(result, expect);
            });
        }
    }
}
