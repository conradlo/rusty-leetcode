pub fn is_valid(s: String) -> bool {
    // println!("{}", s);
    let char_count = s.chars().count();
    if char_count == 0 {
        return true;
    }
    if char_count % 2 != 0 {
        return false;
    }
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        if c == '[' || c == '(' || c == '{' {
            stack.push(c);
        } else if let Some(&last) = stack.last() {
            let open = if c == ']' {
                '['
            } else if c == ')' {
                '('
            } else {
                '{'
            };
            if open == last {
                stack.pop();
            } else {
                return false;
            }
        } else {
            return false;
        }
        // println!("{}", c);
    }
    stack.is_empty()
}

// cargo watch -x "test _0020_ -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_cases() {
        let test_cases = vec![
            ("", true),
            ("(", false),
            ("()[]{}", true),
            ("(]", false),
            ("([)]", false),
            ("{[]}", true),
            ("{[]}()[]{}", true),
            ("({[]}()[]{}}", false),
        ];
        for (input, expect) in test_cases {
            let ans = is_valid(String::from(input));
            assert_eq!(ans, expect);
        }
    }
}
