/*
 * @lc app=leetcode id=119 lang=rust
 *
 * [119] Pascal's Triangle II
 */

struct Solution;
// @lc code=start
impl Solution {
    // simple recursive approach
    // what is the base case?
    // what is the recurrence relation?
    // where is the duplicated computation? (how to avoid?)
    pub fn v1_helper(row_index: usize, item_index: usize, matrix: &mut Vec<Vec<i32>>) -> i32 {
        if item_index == 0 || item_index == row_index {
            return 1;
        }

        let cache_ans = matrix[row_index][item_index];
        if cache_ans != 0 {
            return cache_ans;
        } else {
            let ans = Solution::v1_helper(row_index - 1, item_index - 1, matrix)
                + Solution::v1_helper(row_index - 1, item_index, matrix);

            matrix[row_index][item_index] = ans;

            return ans;
        }
    }
    pub fn get_row_v1(row_index: i32) -> Vec<i32> {
        let row = row_index as usize + 1;
        let mut ans = vec![0; row];
        // need to utilize memoization
        // otherwise it is too slow when row_index == 33
        let mut cache = vec![vec![0; row]; row];
        for i in 0..row {
            ans[i] = Solution::v1_helper(row_index as usize, i, &mut cache);
        }
        ans
    }
}
// @lc code=end

// cargo watch -x "test _0119_ -- --nocapture --test-threads=1"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_test_cases() -> Vec<(i32, Vec<i32>)> {
        return vec![
            (0, vec![1]),
            (1, vec![1, 1]),
            (2, vec![1, 2, 1]),
            (3, vec![1, 3, 3, 1]),
            (4, vec![1, 4, 6, 4, 1]),
        ];
    }

    #[test]
    fn test_get_row() {
        let testcases = get_test_cases();
        for (row_index, expect) in testcases {
            let result = Solution::get_row_v1(row_index);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    pub fn bench_get_row(b: &mut Bencher) {
        b.iter(|| {
            Solution::get_row_v1(33);
        });
    }
}
