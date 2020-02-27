/*
 * @lc app=leetcode id=509 lang=rust
 *
 * [509] Fibonacci Number
 */

struct Solution;
// @lc code=start
impl Solution {
    // base cases: F(0) = 0, F(1) = 1
    // recurrence relation: F(N) = F(N - 1) + F(N - 2), for N > 1.

    // 0 <= N <= 30

    // what is the runtime of pure recursion?
    pub fn fib_v1(n: i32) -> i32 {
        // base case
        if n < 2 {
            return n;
        }
        // recurrence relation
        Solution::fib_v1(n - 1) + Solution::fib_v1(n - 2)
    }

    // recursion + memoization
    pub fn v2_helper(n: i32, cache: &mut Vec<i32>) -> i32 {
        // base case
        if n < 2 {
            return n;
        }
        // memoization
        let usize_n = n as usize;
        if cache[usize_n] != -1 {
            return cache[usize_n];
        }
        // recurrence relation
        let ans = Solution::v2_helper(n - 1, cache) + Solution::v2_helper(n - 2, cache);
        cache[usize_n] = ans;
        ans
    }

    pub fn fib_v2(n: i32) -> i32 {
        let mut cache = vec![-1; n as usize + 1];
        Solution::v2_helper(n, &mut cache)
    }

    pub fn fib_v3(n: i32) -> i32 {
        // bottom up iterative with stack
        // kind of like postorder DFS
        if n < 2 {
            return n;
        }
        let mut fibs: Vec<i32> = vec![-1; n as usize + 1];
        // base case
        fibs[0] = 0;
        fibs[1] = 1;
        let mut stack: Vec<i32> = vec![n];
        while let Some(k) = stack.pop() {
            let usize_k = k as usize;
            if fibs[usize_k - 1] == -1 {
                // left subtree (k - 1)
                stack.push(k);
                stack.push(k - 1);
            } else if fibs[usize_k - 2] == -1 {
                // right subtree (k - 2)
                stack.push(k);
                stack.push(k - 2);
            } else {
                // both left and right subtree has ans
                // recurrence relation
                fibs[usize_k] = fibs[usize_k - 1] + fibs[usize_k - 2];
            }
        }
        // println!("{:?}", fibs);
        fibs[n as usize]
    }

    pub fn fib_v4(n: i32) -> i32 {
        // kind of like bottom up dp
        // trivial case
        if n < 2 {
            return n;
        }
        let usize_n = n as usize;
        // full size vector, save time on pushing/expanding vector
        let mut fibs: Vec<i32> = vec![0; usize_n + 1];
        fibs[1] = 1;
        // integer suffix
        // ..= in rust means inclusive range end
        for i in 2usize..=usize_n {
            fibs[i] = fibs[i - 1] + fibs[i - 2];
        }
        fibs[usize_n]
    }

    pub fn fib_v5(n: i32) -> i32 {
        // like v4 but only rmb lastest 2 values in the sequence
        // trivial case
        if n < 2 {
            return n;
        }
        let mut fib_pair: (i32, i32) = (0, 1);
        for _ in 2..=n {
            fib_pair = (fib_pair.1, fib_pair.0 + fib_pair.1);
        }
        fib_pair.1
    }

    pub fn fib_v6(n: i32) -> i32 {
        // use a pre-calculated table + v5
        // don't use vector, heap allocation cost time (~40-20ns)
        // use an array instead
        let pre_cal = [
            0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181,
            6765, 10946, 17711, 28657, 46368, 75025, 121_393, 196_418, 317_811, 514_229, 832_040,
        ];
        if n < 31 {
            return pre_cal[n as usize];
        }
        let mut fib_pair: (i32, i32) = (514_229, 832_040);
        for _ in 31..=n {
            fib_pair = (fib_pair.1, fib_pair.0 + fib_pair.1);
        }
        fib_pair.1
    }

    // Matrix Exponentiation {O(logN)} time & space
    // Math, golden ratio, Binet's formula {O(1)}
}
// @lc code=end

// cargo watch -x "test _0509_ -- --nocapture --test-threads=1"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    fn get_test_cases() -> Vec<(i32, i32)> {
        vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 8),
            (7, 13),
            (8, 21),
            (9, 34),
            (10, 55),
            (11, 89),
            (12, 144),
            (13, 233),
            (14, 377),
            (15, 610),
            (16, 987),
            (17, 1597),
            (18, 2584),
            (19, 4181),
            (20, 6765),
            (21, 10946),
            (22, 17711),
            (23, 28657),
            (24, 46368),
            (25, 75025),
            (26, 121_393),
            (27, 196_418),
            (28, 317_811),
            (29, 514_229),
            (30, 832_040),
            (31, 1_346_269),
            (32, 2_178_309),
            (40, 102_334_155),
        ]
    }

    #[test]
    #[ignore]
    fn test_fib_v1() {
        for (input, expect) in get_test_cases() {
            let result = Solution::fib_v1(input);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_fib_v2() {
        for (input, expect) in get_test_cases() {
            let result = Solution::fib_v2(input);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_fib_v3() {
        for (input, expect) in get_test_cases() {
            let result = Solution::fib_v3(input);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_fib_v4() {
        for (input, expect) in get_test_cases() {
            let result = Solution::fib_v4(input);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_fib_v5() {
        for (input, expect) in get_test_cases() {
            let result = Solution::fib_v5(input);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_fib_v6() {
        for (input, expect) in get_test_cases() {
            let result = Solution::fib_v6(input);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    fn bench_fib_v1(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::fib_v1(30);
            assert_eq!(result, 832_040);
        });
    }

    #[bench]
    fn bench_fib_v2(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::fib_v2(30);
            assert_eq!(result, 832_040);
        });
    }

    #[bench]
    fn bench_fib_v3(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::fib_v3(30);
            assert_eq!(result, 832_040);
        });
    }

    #[bench]
    fn bench_fib_v4(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::fib_v4(30);
            assert_eq!(result, 832_040);
        });
    }

    #[bench]
    fn bench_fib_v5(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::fib_v5(30);
            assert_eq!(result, 832_040);
        });
    }

    #[bench]
    fn bench_fib_v6(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::fib_v6(30);
            assert_eq!(result, 832_040);
        });
    }
}
