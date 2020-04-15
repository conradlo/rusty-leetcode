/*
 * @lc app=leetcode id=377 lang=rust
 *
 * [377] Combination Sum IV
 */
struct Solution {}

use std::cmp::Ordering;
use std::collections::HashMap;
// @lc code=start
impl Solution {
    // DFS exhausive
    // TLE and slow
    pub fn combination_sum4_v1(nums: Vec<i32>, target: i32) -> i32 {
        // (index of nums, visit flag, accumulate sum)
        let mut stack: Vec<(usize, bool, i32)> = vec![(0, false, 0)];
        let mut count = 0;
        while let Some((i, visited, sum)) = stack.pop() {
            if visited {
                stack.push((i + 1, false, sum - nums[i]));
            } else if i < nums.len() && sum < target {
                let new_sum = sum + nums[i];
                stack.push((i, true, new_sum));
                stack.push((0, false, new_sum));
            } else if sum == target {
                count += 1;
                // println!(
                //     "{:?}",
                //     stack.iter().map(|(i, _, _)| nums[*i]).collect::<Vec<i32>>()
                // );
            }
        }
        count
    }

    // recursive DFS
    // still slow but easier to write
    // and easy to optimise
    pub fn v2_helper(nums: &[i32], target: i32) -> i32 {
        match target.cmp(&0) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => {
                let mut ans = 0;
                for num in nums.iter() {
                    ans += Solution::v2_helper(nums, target - num);
                }
                ans
            }
        }
    }

    pub fn combination_sum4_v2(nums: Vec<i32>, target: i32) -> i32 {
        Solution::v2_helper(&nums, target)
    }

    // optimized version of v2
    // recursive DFS + memoization
    // fast!
    pub fn v3_helper(nums: &[i32], target: i32, memo: &mut HashMap<i32, i32>) -> i32 {
        if let Some(cache) = memo.get(&target) {
            return *cache;
        }
        let ans = match target.cmp(&0) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => {
                let mut count = 0;
                for num in nums.iter() {
                    count += Solution::v3_helper(nums, target - num, memo);
                }
                count
            }
        };
        memo.insert(target, ans);
        ans
    }

    pub fn combination_sum4_v3(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        Solution::v3_helper(&nums, target, &mut memo)
    }

    // bottom up, "pull" dp
    // O(c * n)
    pub fn combination_sum4_v4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp: Vec<i32> = vec![0; target + 1];
        dp[0] = 1;
        for i in 1..=target {
            for &num in &nums {
                if i >= num as usize {
                    // "pull" from previous subproblems'
                    dp[i] += dp[i - num as usize];
                }
            }
        }
        dp.pop().unwrap()
    }

    // bottom up, "forward"/"push" dp
    pub fn combination_sum4_v5(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0; target + 1];
        dp[0] = 1;
        for i in 0..=target {
            for &num in nums.iter() {
                let x = num as usize;
                if i + x < dp.len() {
                    // "push" i's subproblem into future i + x's
                    dp[i + x] += dp[i];
                }
            }
        }
        //
        dp.pop().unwrap()
    }
}
// @lc code=end

// cargo watch -x "test _0377_ -- --nocapture"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_test_cases() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![1, 2, 3], 4, 7),
            (vec![1, 2, 3], 10, 274),
            (vec![2, 4, 6], 12, 24),
            (vec![1, 2, 3], 16, 10_609),
        ]
    }

    fn get_test_cases_large() -> Vec<(Vec<i32>, i32, i32)> {
        vec![
            (vec![1, 2, 3], 32, 181_997_601),
            (vec![1, 2, 4], 32, 39_882_198),
        ]
    }

    #[bench]
    fn run_test_cases_v1_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v1(nums.clone(), target);
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
                let result = Solution::combination_sum4_v2(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }

    #[bench]
    #[ignore]
    fn run_test_cases_v2_and_bench_large(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases_large();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v2(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }

    #[bench]
    fn run_test_cases_v3_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v3(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }
    #[bench]
    fn run_test_cases_v3_and_bench_large(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases_large();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v3(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }
    #[bench]
    fn run_test_cases_v4_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v4(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }
    #[bench]
    fn run_test_cases_v4_and_bench_large(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases_large();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v4(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }
    #[bench]
    fn run_test_cases_v5_and_bench(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v5(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }
    #[bench]
    fn run_test_cases_v5_and_bench_large(b: &mut Bencher) {
        // (nums, target, expect)
        let testcases = get_test_cases_large();
        for (nums, target, expect) in testcases {
            b.iter(|| {
                let result = Solution::combination_sum4_v5(nums.clone(), target);
                assert_eq!(result, expect);
            });
        }
    }
}
