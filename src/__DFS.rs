// use std::collections::HashMap;

pub fn DFS(matrix: Vec<Vec<i32>>, sr: i32, sc: i32) {
    let row_count = matrix.len() as i32;
    let col_count = matrix[0].len() as i32;

    let mut visited = vec![vec![-1; matrix[0].len()]; matrix.len()];
    let mut stack = Vec::new();

    stack.push((sr, sc));
    while stack.len() > 0 {
        let top = stack.last().unwrap();
        let r = top.0;
        let c = top.1;
        visited[r as usize][c as usize] = 1;
        match [(r, c + 1), (r + 1, c), (r, c - 1), (r - 1, c)]
            .iter()
            .find(|pair| {
                pair.0 >= 0
                    && pair.0 < row_count
                    && pair.1 >= 0
                    && pair.1 < col_count
                    && visited[pair.0 as usize][pair.1 as usize] == -1
            }) {
            Some(pair) => stack.push(*pair),
            None => {
                println!("{}:{} | {}", r, c, matrix[r as usize][c as usize]);
                stack.pop();
            }
        }
    }
    println!("------");
}

// cargo watch -x "test _733_flood_fill -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn run_DFS() {
        let test_cases = vec![
            (vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], (1, 1)),
            (vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8]], (1, 1)),
            (
                vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]],
                (4, 1),
            ),
        ];
        for (matrix, args) in test_cases {
            DFS(matrix, args.0, args.1);
            // assert_eq!(ans, expect);
        }
    }
}
