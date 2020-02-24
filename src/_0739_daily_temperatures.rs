pub fn daily_temperatures(t: Vec<i32>) -> Vec<i32> {
    let l = t.len();
    let mut ans = vec![0; l];
    if l < 2 {
        return ans;
    }
    let mut stack: Vec<(usize, i32)> = vec![];
    for (idx, &tem) in t.iter().enumerate() {
        while !stack.is_empty() {
            if let Some(&top) = stack.last() {
                if tem > top.1 {
                    ans[top.0] = (idx - top.0) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
        }
        stack.push((idx, tem));
    }
    ans
}

// cargo watch -x "test _0739_ -- --nocapture"
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test_cases() {
        let test_cases = vec![
            (vec![100], vec![0]),
            (
                vec![73, 74, 75, 71, 69, 72, 76, 73],
                vec![1, 1, 4, 2, 1, 1, 0, 0],
            ),
            (
                vec![100, 75, 73, 74, 75, 71, 69, 72, 76, 73],
                vec![0, 7, 1, 1, 4, 2, 1, 1, 0, 0],
            ),
            (
                vec![100, 100, 100, 100, 100, 100, 100, 100],
                vec![0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (
                vec![100, 35, 34, 33, 32, 31, 30, 36],
                vec![0, 6, 5, 4, 3, 2, 1, 0],
            ),
        ];
        for (input, expect) in test_cases {
            let ans = daily_temperatures(input);
            assert_eq!(ans, expect);
        }
    }
}
