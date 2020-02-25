/*
 * @lc app=leetcode id=542 lang=rust
 *
 * [542] 01 Matrix
 */
struct Solution;

// @lc code=start
use std::collections::VecDeque;
impl Solution {
    // 32ms 3.1 MB
    pub fn update_matrix_v1(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // BFS on each cell O(n^2)
        let row_count = matrix.len();
        if row_count < 1 {
            return vec![];
        }
        let col_count = matrix[0].len();
        let mut ans = vec![vec![-1; col_count]; row_count];
        for (r_idx, row) in ans.iter_mut().enumerate() {
            for (c_idx, cell) in row.iter_mut().enumerate() {
                // println!("({},{}): {}", r_idx, c_idx, matrix[r_idx][c_idx]);
                let mut q: VecDeque<(i32, (usize, usize))> = VecDeque::new();
                q.push_back((0, (r_idx, c_idx)));
                while let Some((step, (r, c))) = q.pop_front() {
                    if matrix[r][c] == 0 {
                        // println!("> ({}, {}) : {}", r, c, step);
                        // ans[r_idx][c_idx] = step;
                        *cell = step;
                        break;
                    }
                    if r + 1 < row_count {
                        q.push_back((step + 1, (r + 1, c)));
                    }
                    if r > 0 {
                        q.push_back((step + 1, (r - 1, c)));
                    }
                    if c + 1 < col_count {
                        q.push_back((step + 1, (r, c + 1)));
                    }
                    if c > 0 {
                        q.push_back((step + 1, (r, c - 1)));
                    }
                }
            }
        }
        ans
    }
    pub fn update_matrix_v2(_matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // if matrix[r][c] = 0 then ans[r][c] = 0
        // else ans[r][c] = 1 + min(matrix[r-1][c], matrix[r+1][c], matrix[r][c-1], matrix[r][c+1])
        vec![]
    }
}
// @lc code=end

// cargo watch -x "test _0542_"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use test::Bencher;

    type Input = Vec<Vec<i32>>;
    type Expect = Vec<Vec<i32>>;

    fn get_test_cases() -> Vec<(Input, Expect)> {
        // (input, expect)
        vec![
            (vec![], vec![]),
            (vec![vec![]], vec![vec![]]),
            (vec![vec![0]], vec![vec![0]]),
            (vec![vec![0, 1, 1]], vec![vec![0, 1, 2]]),
            (
                vec![vec![0], vec![1], vec![1]],
                vec![vec![0], vec![1], vec![2]],
            ),
            (
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]],
            ),
            (
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]],
            ),
            (
                vec![vec![0, 0, 1, 0], vec![0, 1, 1, 1]],
                vec![vec![0, 0, 1, 0], vec![0, 1, 2, 1]],
            ),
            (
                vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 0]],
                vec![vec![5, 4, 3], vec![4, 3, 2], vec![3, 2, 1], vec![2, 1, 0]],
            ),
            (
                vec![
                    vec![0, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                ],
                vec![
                    vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                    vec![1, 2, 3, 4, 3, 4, 5, 6, 7, 8],
                    vec![2, 3, 4, 3, 2, 3, 4, 5, 6, 7],
                    vec![3, 4, 3, 2, 1, 2, 3, 4, 5, 6],
                    vec![4, 3, 2, 1, 0, 1, 2, 3, 4, 5],
                    vec![5, 4, 3, 2, 1, 2, 3, 4, 5, 4],
                    vec![6, 5, 4, 3, 2, 3, 4, 5, 4, 3],
                    vec![7, 6, 5, 4, 3, 4, 5, 4, 3, 2],
                    vec![8, 7, 6, 5, 4, 5, 4, 3, 2, 1],
                    vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0],
                ],
            ),
        ]
    }

    #[test]
    pub fn test_update_matrix_v1() {
        for (input, expect) in get_test_cases() {
            let result = Solution::update_matrix_v1(input);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    pub fn bench_update_matrix_v1(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_matrix: Vec<Vec<i32>> = vec![vec![0; 100]; 100];
        for row in rand_matrix.iter_mut() {
            for entry in row.iter_mut() {
                *entry = rng.gen_range(0, 1);
            }
        }
        b.iter(|| {
            Solution::update_matrix_v1(rand_matrix.clone());
        });
    }
}
