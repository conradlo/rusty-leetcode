/*
 * @lc app=leetcode id=989 lang=rust
 *
 * [989] Add to Array-Form of Integer
 */

struct Solution;
// @lc code=start
impl Solution {
    pub fn add_to_array_form(a: Vec<i32>, k: i32) -> Vec<i32> {
        // 0 <= K <= 10000
        if k == 0 {
            return a;
        }
        let mut a = a;
        let mut k = k;
        // split K
        let mut b: Vec<i32> = vec![];
        while k > 0 {
            b.push(k % 10);
            k /= 10;
        }
        b.reverse();

        // 1 <= A.len <= 10000

        let length = if a.len() > b.len() { a.len() } else { b.len() } + 1;
        let mut ans: Vec<i32> = vec![0; length];

        let mut i: usize = length - 1;
        let mut carrier: i32 = 0;
        while !(a.is_empty() && b.is_empty() && carrier == 0) {
            let sum = match (a.pop(), b.pop()) {
                (Some(a_val), Some(b_val)) => a_val + b_val + carrier,
                (None, Some(b_val)) => b_val + carrier,
                (Some(a_val), None) => a_val + carrier,
                (None, None) => carrier,
            };

            carrier = if sum > 9 { 1 } else { 0 };
            ans[i] = sum % 10;
            if i > 0 {
                // prevent overflow
                i -= 1;
            }
        }
        // println!("{:#?}", ans);
        if ans[0] == 0 {
            ans[1..].to_vec()
        } else {
            ans
        }
    }
}
// @lc code=end

// cargo watch -x "test _0989_ -- --nocapture --test-threads=1"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    type Input = (Vec<i32>, i32);
    type Expect = Vec<i32>;
    fn get_test_cases() -> Vec<(Input, Expect)> {
        vec![
            ((vec![1], 0), vec![1]),
            ((vec![1, 2, 3, 4], 10000), vec![1, 1, 2, 3, 4]),
            ((vec![1, 2, 0, 0], 34), vec![1, 2, 3, 4]),
            ((vec![2, 7, 4], 181), vec![4, 5, 5]),
            ((vec![2, 1, 5], 806), vec![1, 0, 2, 1]),
            ((vec![9, 9, 9], 999), vec![1, 9, 9, 8]),
            ((vec![8, 7, 6, 5, 4], 12345), vec![9, 9, 9, 9, 9]),
            ((vec![7, 6, 5, 4], 14345), vec![2, 1, 9, 9, 9]),
            (
                (vec![9, 9, 9, 9, 9, 9, 9, 9, 9, 9], 1),
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ),
        ]
    }

    #[test]
    fn test_v1() {
        //
        for ((a, k), expect) in get_test_cases() {
            let result = Solution::add_to_array_form(a, k);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    fn bench_and_test(b: &mut Bencher) {
        //
        let a: Vec<i32> = vec![9; 10_000];
        let k = 1;
        let mut expect: Vec<i32> = vec![0; 10_001];
        expect[0] = 1;
        b.iter(|| {
            let result = Solution::add_to_array_form(a.clone(), k);
            assert_eq!(result, expect);
        });
    }
}
