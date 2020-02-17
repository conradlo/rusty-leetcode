/*
 * @lc app=leetcode id=344 lang=rust
 *
 * [344] Reverse String
 */

struct Solution;
// @lc code=start
impl Solution {
    // recursion
    // in place
    // Space O(n) -> change to iterative then O(1)
    // Time O(n)

    pub fn helper_v1(i: usize, j: usize, s: &mut Vec<char>) {
        if i < j {
            let tmp = s[i];
            s[i] = s[j];
            s[j] = tmp;
            Solution::helper_v1(i + 1, j - 1, s);
        }
    }
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() > 0 {
            Solution::helper_v1(0, s.len() - 1, s);
        }
    }
}
// @lc code=end

// cargo watch -x "test _0344_ -- --nocapture"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test_cases_v1() {
        let testcases: Vec<(Vec<char>, Vec<char>)> = vec![
            (vec![], vec![]),
            (vec!['A'], vec!['A']),
            (vec!['A', 'b'], vec!['b', 'A']),
            (vec!['h', 'e', 'l', 'l', 'o'], vec!['o', 'l', 'l', 'e', 'h']),
            (
                vec!['H', 'a', 'n', 'n', 'a', 'h'],
                vec!['h', 'a', 'n', 'n', 'a', 'H'],
            ),
        ];
        for (input, expect) in testcases {
            let mut result = input.clone();
            Solution::reverse_string(&mut result);
            // println!("{:?}", result);
            assert_eq!(result, expect);
        }
    }
}
