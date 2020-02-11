pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack: Vec<i32> = vec![];
    for token in tokens {
        match token.as_str() {
            "+" => {
                let snd = stack.pop().unwrap();
                let fst = stack.pop().unwrap();
                stack.push(fst + snd);
            }
            "-" => {
                let snd = stack.pop().unwrap();
                let fst = stack.pop().unwrap();
                stack.push(fst - snd);
            }
            "*" => {
                let snd = stack.pop().unwrap();
                let fst = stack.pop().unwrap();
                stack.push(fst * snd);
            }
            "/" => {
                let snd = stack.pop().unwrap();
                let fst = stack.pop().unwrap();
                stack.push(fst / snd);
            }
            _ => {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
    }
    stack.pop().unwrap()
}

// cargo watch -x "test _0150_ -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_cases() {
        let testcases = vec![
            (vec!["2", "1", "+", "3", "*"], 9),
            (vec!["4", "13", "5", "/", "+"], 6),
            (
                vec![
                    "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
                ],
                22,
            ),
        ];
        for (input, expect) in testcases {
            let result = eval_rpn(input.iter().map(|&val| String::from(val)).collect());
            assert_eq!(result, expect);
        }
    }
}
