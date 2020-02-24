// use std::collections::HashMap;
pub fn dfs(matrix: Vec<Vec<i32>>, sr: i32, sc: i32) {
    let row_count = matrix.len();
    let col_count = if row_count > 0 { matrix[0].len() } else { 0 };

    let mut visited = vec![vec![0; col_count]; row_count];
    let mut stack = Vec::new();

    stack.push((sr as usize, sc as usize));
    while !stack.is_empty() {
        if let Some(&top) = stack.last() {
            let (r, c) = top;
            visited[r][c] = 1;
            if r > 0 && visited[r - 1][c] == 0 {
                stack.push((r - 1, c));
            } else if r + 1 < row_count && visited[r + 1][c] == 0 {
                stack.push((r + 1, c));
            } else if c > 0 && visited[r][c - 1] == 0 {
                stack.push((r, c - 1));
            } else if c + 1 < col_count && visited[r][c + 1] == 0 {
                stack.push((r, c + 1));
            } else {
                println!("{}:{} | {}", r, c, matrix[r][c]);
                stack.pop();
            }
        }
    }
    println!("------");
}

// cargo watch -x "test matrix_traverse_dfs -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    // #[ignore]
    fn run_test() {
        let test_cases = vec![
            (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], (1, 1)),
            (vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]], (1, 1)),
            (
                vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]],
                (4, 1),
            ),
        ];
        for (matrix, args) in test_cases {
            dfs(matrix, args.0, args.1);
            // assert_eq!(ans, expect);
        }
    }
}
