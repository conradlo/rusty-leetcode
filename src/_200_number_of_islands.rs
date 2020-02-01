use std::collections::VecDeque;
#[allow(dead_code)]
pub fn bfs(matrix: &mut Vec<Vec<char>>, row: usize, col: usize, sr: usize, sc: usize) {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((sr, sc));
    while !queue.is_empty() {
        let (r, c) = queue.pop_front().unwrap();
        if matrix[r][c] == '1' {
            matrix[r][c] = '0';
            if r + 1 < row {
                queue.push_back((r + 1, c));
            }
            if r > 0 {
                queue.push_back((r - 1, c));
            }
            if c + 1 < col {
                queue.push_back((r, c + 1));
            }
            if c > 0 {
                queue.push_back((r, c - 1));
            }
        }
    }
}
#[allow(dead_code)]
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid;
    if grid.len() == 0 {
        return 0;
    }
    let row_count = grid.len();
    let col_count = grid[0].len();
    //
    let mut ans = 0;
    for row_idx in 0..row_count {
        for col_idx in 0..col_count {
            if grid[row_idx][col_idx] == '1' {
                ans += 1;
                bfs(&mut grid, row_count, col_count, row_idx, col_idx);
            }
        }
    }
    ans
}
// cargo watch -x "test _200_number_of_islands -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_cases() {
        let test_cases = vec![
            (vec![], 0),
            (vec![vec![]], 0),
            (vec![vec!['0']], 0),
            (vec![vec!['1', '0'], vec!['0', '1']], 2),
            (
                vec![
                    vec!['1', '0', '1'],
                    vec!['0', '1', '0'],
                    vec!['1', '0', '1'],
                ],
                5,
            ),
            (
                vec![
                    vec!['1', '1', '1', '1', '0'],
                    vec!['1', '1', '0', '1', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                1,
            ),
            (
                vec![
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                3,
            ),
            (
                vec![
                    vec!['1', '1', '0', '0', '1'],
                    vec!['1', '1', '1', '0', '1'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['1', '1', '0', '1', '1'],
                ],
                4,
            ),
            (
                vec![
                    vec!['1', '1', '0', '0', '1', '0', '0', '0', '0', '1'],
                    vec!['1', '1', '1', '1', '1', '0', '0', '1', '0', '1'],
                    vec!['0', '0', '0', '0', '1', '0', '1', '1', '0', '0'],
                    vec!['1', '0', '0', '0', '0', '0', '0', '0', '1', '0'],
                    vec!['1', '0', '1', '0', '0', '0', '1', '0', '0', '1'],
                    vec!['1', '0', '0', '1', '0', '1', '0', '0', '1', '1'],
                    vec!['1', '1', '0', '0', '0', '1', '0', '1', '1', '1'],
                ],
                10,
            ),
        ];
        for (input, expect) in test_cases {
            let ans = num_islands(input);
            assert_eq!(ans, expect);
        }
    }
}
