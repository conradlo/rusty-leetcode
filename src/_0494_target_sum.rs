/*
 * @lc app=leetcode id=494 lang=rust
 *
 * [494] Target Sum
 */

struct Solution;
// @lc code=start
impl Solution {
    // start of v1
    fn all_sum_ways_v1(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        // exhaustively list all possible sums
        // could be slow
        if nums.is_empty() {
            return ans;
        } else if nums.len() == 1 {
            ans.push(nums[0]);
            ans.push(-nums[0]); // can just add a `-` sign: https://doc.rust-lang.org/std/ops/trait.Neg.html
            return ans;
        }
        let mut nums = nums;
        if let Some(last) = nums.pop() {
            let mut left: Vec<i32> = Solution::all_sum_ways_v1(nums.clone())
                .iter()
                .map(|val| val + last)
                .collect();
            let mut right: Vec<i32> = Solution::all_sum_ways_v1(nums)
                .iter()
                .map(|val| val - last)
                .collect();
            ans.append(&mut left);
            ans.append(&mut right);
        }
        ans
    }
    pub fn find_target_sum_ways_v1(nums: Vec<i32>, s: i32) -> i32 {
        // build tree
        // traverse
        // check accumulated sum
        // this answer is slow and exceed time limit
        let mut ans = 0;
        if !nums.is_empty() {
            let all_sums = Solution::all_sum_ways_v1(nums);
            // println!("{:?}", all_sums);
            for val in all_sums {
                if val == s {
                    ans += 1;
                }
            }
        }
        ans
    }
    // end of v1

    // start of v2
    fn all_sum_ways_v2(nums: Vec<i32>) -> Vec<i32> {
        // DFS post order iterative
        let mut all_sums = vec![];
        if !nums.is_empty() {
            // cursor is the index of `nums` i.e. [0..nums.len()]
            let head = nums[0];
            // (examined, cursor, acc_sum);
            let mut stack: Vec<(bool, usize, i32)> = vec![(false, 0, 0 - head), (false, 0, head)];
            while let Some((examined, cursor, acc_sum)) = stack.pop() {
                if !examined {
                    if cursor + 1 < nums.len() {
                        stack.push((true, cursor, acc_sum));
                        let next_cursor = cursor + 1;
                        stack.push((false, next_cursor, acc_sum - nums[next_cursor]));
                        stack.push((false, next_cursor, acc_sum + nums[next_cursor]));
                    } else {
                        // cursor + 1 exceed nums's len, so this is a leaf node
                        all_sums.push(acc_sum);
                    }
                }
            }
        }
        all_sums
    }

    pub fn find_target_sum_ways_v2(nums: Vec<i32>, s: i32) -> i32 {
        // DFS post order iterative
        let mut count = 0;
        if !nums.is_empty() {
            // cursor is the index of `nums` i.e. [0..nums.len()]
            let head = nums[0];
            // (examined, cursor, acc_sum);
            let mut stack: Vec<(bool, usize, i32)> = vec![(false, 0, 0 - head), (false, 0, head)];
            while let Some((examined, cursor, acc_sum)) = stack.pop() {
                if !examined {
                    if cursor + 1 < nums.len() {
                        stack.push((true, cursor, acc_sum));
                        let next_cursor = cursor + 1;
                        stack.push((false, next_cursor, acc_sum - nums[next_cursor]));
                        stack.push((false, next_cursor, acc_sum + nums[next_cursor]));
                    } else if acc_sum == s {
                        // cursor + 1 exceed nums's len, so this is a leaf node
                        // check target sum
                        count += 1;
                    }
                }
            }
        }
        count
    }
    // end of v2

    // v3

    // O(2^n) (520ms) ~30%
    pub fn v3_helper(i: usize, nums: &[i32], target: i32, acc_sum: i32) -> i32 {
        if i < nums.len() {
            let left = Solution::v3_helper(i + 1, nums, target, acc_sum + nums[i]);
            let right = Solution::v3_helper(i + 1, nums, target, acc_sum - nums[i]);
            return left + right; // post order
        }
        if target == acc_sum {
            1
        } else {
            0
        }
    }
    pub fn find_target_sum_ways_v3(nums: Vec<i32>, s: i32) -> i32 {
        // recursive implementation of v2
        Solution::v3_helper(0, &nums, s, 0)
    }

    // v4 (8ms/ 75%)
    pub fn v4_helper(
        i: usize,
        nums: &[i32],
        target: i32,
        acc_sum: i32,
        matrix: &mut Vec<Vec<i32>>,
    ) -> i32 {
        if i < nums.len() {
            if matrix[i][(acc_sum + 1000) as usize] != -1 {
                return matrix[i][(acc_sum + 1000) as usize];
            }
            let left = Solution::v4_helper(i + 1, nums, target, acc_sum + nums[i], matrix);
            let right = Solution::v4_helper(i + 1, nums, target, acc_sum - nums[i], matrix);
            let ans = left + right;
            matrix[i][(acc_sum + 1000) as usize] = ans;
            return ans; // post order || bottom to top
        }
        if target == acc_sum {
            1
        } else {
            0
        }
    }
    pub fn find_target_sum_ways_v4(nums: Vec<i32>, s: i32) -> i32 {
        // v3 with memoization
        let i = nums.len();
        let j = 2 * 1000 + 1; // -1000 ... 0 ... 1000
        let mut matrix = vec![vec![-1; j]; i];
        // println!("{:?}", matrix);
        Solution::v4_helper(0, &nums, s, 0, &mut matrix)
    }
}
// @lc code=end

// cargo watch -x "test _0494_ -- --nocapture --test-threads=1"
// use rand;
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use test::Bencher;

    #[test]
    fn run_test_cases_all_sum_v1() {
        let testcases: Vec<(Vec<i32>, Vec<i32>)> = vec![
            (vec![], vec![]),
            (vec![1], vec![1, -1]),
            (vec![1, 1], vec![2, 0, 0, -2]),
            (vec![1, 1, 1], vec![3, 1, 1, -1, 1, -1, -1, -3]),
            (vec![1, 2], vec![3, 1, -1, -3]),
            (vec![1, 2, 3], vec![6, 4, 2, 0, 0, -2, -4, -6]),
            (
                vec![1, 1, 1, 1, 1],
                vec![
                    5, 3, 3, 1, 3, 1, 1, -1, 3, 1, 1, -1, 1, -1, -1, -3, 3, 1, 1, -1, 1, -1, -1,
                    -3, 1, -1, -1, -3, -1, -3, -3, -5,
                ],
            ),
        ];
        for (input, expect) in testcases {
            let result = Solution::all_sum_ways_v1(input);
            // println!("{:?} : {:?}", input, expect);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn run_test_cases_v1() {
        // (nums, target, expect)
        let testcases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![1, 1, 1], 3, 1),
            (vec![1, 2, 3], 10, 0),
            (vec![1, 1, 1, 1, 1], 3, 5),
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 55, 1),
            // (
            //     vec![
            //         34, 16, 5, 38, 20, 20, 8, 43, 3, 46, 24, 12, 28, 19, 22, 28, 9, 46, 25, 36,
            //     ],
            //     0,
            //     6638,
            // ),
        ];
        for (nums, s, expect) in testcases {
            let result = Solution::find_target_sum_ways_v1(nums, s);
            assert_eq!(result, expect);
        }
    }

    // cargo bench _0494_
    #[bench]
    fn benc_all_sum_ways_v1(b: &mut Bencher) {
        b.iter(|| {
            // around 150,000,000 ns/iter (150 ms/iter)
            // let input = vec![
            //     34, 16, 5, 38, 20, 20, 8, 43, 3, 46, 24, 12, 28, 19, 22, 28, 9, 46, 25, 36,
            // ];
            // around 2, 000, 000 ns /  tier
            let input = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
            Solution::all_sum_ways_v1(input);
        })
    }

    #[bench]
    #[ignore]
    // slow
    fn bench_find_target_sum_ways_v1(b: &mut Bencher) {
        b.iter(|| {
            let input = vec![
                34, 16, 5, 38, 20, 20, 8, 43, 3, 46, 24, 12, 28, 19, 22, 28, 9, 46, 25, 36,
            ];
            Solution::find_target_sum_ways_v1(input, 0);
        });
    }

    // v2
    #[test]
    fn run_test_cases_v2() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let n: usize = rng.gen_range(0, 15);
            // println!("{}", n);
            let mut rand_arr: Vec<i32> = vec![0; n];
            for rand_num in rand_arr.iter_mut() {
                *rand_num = rng.gen_range(1, 99);
            }
            let k: i32 = rng.gen_range(-100, 100);
            let ans_1 = Solution::find_target_sum_ways_v1(rand_arr.clone(), k);
            let result = Solution::find_target_sum_ways_v2(rand_arr.clone(), k);
            // println!("{} : {}", ans_1, result);
            assert_eq!(result, ans_1);
        }
    }

    #[test]
    fn run_test_cases_all_sum_v2() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let n: usize = rng.gen_range(0, 15);
            // println!("{}", n);
            let mut rand_arr: Vec<i32> = vec![0; n];
            for rand_num in rand_arr.iter_mut() {
                *rand_num = rng.gen_range(1, 99);
            }
            let mut ans_1 = Solution::all_sum_ways_v1(rand_arr.clone());
            ans_1.sort_unstable();
            let mut result = Solution::all_sum_ways_v2(rand_arr.clone());
            result.sort_unstable();
            assert_eq!(result, ans_1);
        }
    }

    #[bench]
    fn benc_all_sum_ways_v2(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_arr: Vec<i32> = vec![0; 20];
        for rand_num in rand_arr.iter_mut() {
            *rand_num = rng.gen_range(1, 100);
        }
        // n = 20, around 10,000,000 ns/iter (~10ms)
        b.iter(|| {
            Solution::all_sum_ways_v2(rand_arr.clone());
        })
    }

    #[bench]
    // faster
    fn bench_find_target_sum_ways_v2(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_arr: Vec<i32> = vec![0; 20];
        for rand_num in rand_arr.iter_mut() {
            *rand_num = rng.gen_range(1, 100);
        }
        b.iter(|| {
            Solution::find_target_sum_ways_v2(rand_arr.clone(), 0);
        });
    }
    #[test]
    fn run_test_cases_v3() {
        // (nums, target, expect)
        let testcases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![], 1, 0),
            (vec![1, 0], 1, 2),
            (vec![0, 0, 0, 0, 0], 0, 32),
            (vec![1, 1, 1], 3, 1),
            (vec![1, 2, 3], 10, 0),
            (vec![1, 1, 1, 1, 1], 3, 5),
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 55, 1),
            (
                vec![
                    34, 16, 5, 38, 20, 20, 8, 43, 3, 46, 24, 12, 28, 19, 22, 28, 9, 46, 25, 36,
                ],
                0,
                6638,
            ),
        ];
        for (nums, s, expect) in testcases {
            let result = Solution::find_target_sum_ways_v3(nums, s);
            assert_eq!(result, expect);
        }
    }
    #[bench]
    // faster
    fn bench_find_target_sum_ways_v3(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_arr: Vec<i32> = vec![0; 20];
        for rand_num in rand_arr.iter_mut() {
            *rand_num = rng.gen_range(1, 100);
        }
        b.iter(|| {
            Solution::find_target_sum_ways_v3(rand_arr.clone(), 0);
        });
    }
    #[test]
    fn run_test_cases_v4() {
        // (nums, target, expect)
        let testcases: Vec<(Vec<i32>, i32, i32)> = vec![
            (vec![], 1, 0),
            (vec![1, 0], 1, 2),
            (vec![0, 0, 0, 0, 0], 0, 32),
            (vec![1, 1, 1], 3, 1),
            (vec![1, 2, 3], 10, 0),
            (vec![1, 1, 1, 1, 1], 3, 5),
            (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 55, 1),
            (
                vec![
                    34, 16, 5, 38, 20, 20, 8, 43, 3, 46, 24, 12, 28, 19, 22, 28, 9, 46, 25, 36,
                ],
                0,
                6638,
            ),
        ];
        for (nums, s, expect) in testcases {
            let result = Solution::find_target_sum_ways_v4(nums, s);
            assert_eq!(result, expect);
        }
    }
    #[test]
    fn run_test_cases_v4_random() {
        for _ in 0..50 {
            let mut rng = rand::thread_rng();
            let k: i32 = rng.gen_range(-1000, 1000);
            let n: usize = rng.gen_range(0, 15);
            // println!("{}", n);
            let mut rand_arr: Vec<i32> = vec![0; n];
            for rand_num in rand_arr.iter_mut() {
                *rand_num = rng.gen_range(0, 50);
            }
            let ans_1 = Solution::find_target_sum_ways_v3(rand_arr.clone(), k);
            let ans_2 = Solution::find_target_sum_ways_v3(rand_arr.clone(), k);
            // println!("{} {}", ans_1, ans_2);
            assert_eq!(ans_1, ans_2);
        }
    }

    #[bench]
    // faster
    fn bench_find_target_sum_ways_v4(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_arr: Vec<i32> = vec![0; 20];
        for rand_num in rand_arr.iter_mut() {
            *rand_num = rng.gen_range(0, 50);
        }
        b.iter(|| {
            Solution::find_target_sum_ways_v4(rand_arr.clone(), 0);
        });
    }
}
