// bottom up DP
#[allow(dead_code)]
pub fn num_squares(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let n = n as usize;
    let mut squares = vec![1];
    let mut ans_list = vec![0; n];
    for num in 1..=n {
        let mut min_num_square = num;
        for i in 0..squares.len() {
            let square = squares[i];
            if square == num {
                min_num_square = 1;
                // need more squares
                let l = squares.len() + 1;
                squares.push(l * l);
            } else if square < num {
                let val = 1 + ans_list[num - square - 1];
                if val < min_num_square {
                    min_num_square = val;
                }
            } else {
                break;
            }
        }
        ans_list[num - 1] = min_num_square;
    }
    return ans_list[n - 1] as i32;
}

// cargo watch -x "test _0279_ -- --nocapture"
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test_cases() {
        let test_cases = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (4, 1),
            (5, 2),
            (6, 3),
            (7, 4),
            (8, 2),
            (9, 1),
            (10, 2),
            (11, 3),
            (12, 3),
            (13, 2),
            (37, 2),
            (100, 1),
        ];
        for (input, expect) in test_cases {
            let ans = num_squares(input);
            assert_eq!(ans, expect);
        }
    }
}
