#[allow(dead_code)]
pub fn decode_string(s: String) -> String {
    let mut stack = Vec::<(String, usize)>::new();
    let mut mul = 0;
    let mut reader: String = String::from("");

    for c in s.chars() {
        if c == '[' {
            stack.push((reader.to_owned(), mul));
            mul = 0;
            reader = String::from("");
        } else if c == ']' {
            if let Some((mut prev, n)) = stack.pop() {
                prev.push_str(&reader.repeat(n));
                reader = prev;
            }
        } else if let Some(d) = c.to_digit(10) {
            mul = mul * 10 + (d as usize);
        } else {
            reader.push(c);
        }
    }
    return reader;
}

// cargo watch -x "test _0394_ -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_test_cases() {
        let test_cases = vec![
            ("ab", "ab"),
            ("2[a]", "aa"),
            ("1[a]2[b]3[c]", "abbccc"),
            ("3[a]2[bc]", "aaabcbc"),
            ("3[a2[c]]", "accaccacc"),
            ("2[abc]3[cd]ef", "abcabccdcdcdef"),
            ("10[a]", "aaaaaaaaaa"),
            ("1[a2[b3[c4[d]]]]", "abcddddcddddcddddbcddddcddddcdddd"),
            ("3[a]2[b4[F]c]", "aaabFFFFcbFFFFc"),
            (
                "2[a2[b3[cd]3[ef]]]",
                "abcdcdcdefefefbcdcdcdefefefabcdcdcdefefefbcdcdcdefefef",
            ),
        ];
        for (input, expect) in test_cases {
            let ans = decode_string(String::from(input));
            assert_eq!(ans, expect);
        }
    }
}

// TODO:
