/*
 * @lc app=leetcode id=70 lang=rust
 *
 * [70] Climbing Stairs
 */

struct Solution;
// @lc code=start
impl Solution {
    // 1. mark down key info from questions
    // n: int >= 1
    // {1,2} what if you can use more than {1,2}?
    // no of ways to sum up to n

    // Example
    // n == 1, ans == 1 (cannot use "2", use one "1")
    // n == 2, ans == 2 ("1" + "1", "2")
    // n == 3, ans == 3 ("1" + "1" + "1", "1" + "2", "2" + "1")

    // 2. start with "dump" and straight forward approach
    // "dump approach": list out all combination, count how many of them exist

    // play with simple cases by hand
    // observe any pattern

    // n = 0, ans = 0
    // n = 1, ans = 1 [1]
    // n = 2, ans = 2
    // 1[1]
    // 2
    // n = 3, ans = 3
    // 1[11,2]
    // 2[1]

    // n = 4, ans = 5
    // 1 [111|21|12]
    // 2 [11|2]

    // n = 5, ans = 8
    // 1 [1[111|12|21], 2[11|2]]
    // 2 [111|21|12]
    pub fn climb_stairs(n: i32) -> i32 {
        // base case
        if n < 2 {
            // when n == 0, we used up all stairs
            return 1;
        }
        // recurrence relation
        // what is the time complexity of this recursion algo?
        return Solution::climb_stairs(n - 1) + Solution::climb_stairs(n - 2);
    }

    pub fn climb_stairs_v2(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let mut pair = (1, 2);
        for _ in 2..n {
            pair = (pair.1, pair.0 + pair.1);
        }
        pair.1
    }
}
// @lc code=end

// cargo watch -x "test _0070_ -- --nocapture --test-threads=1"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_climb_stairs() {
        let testcases: Vec<(i32, i32)> = vec![(1, 1), (2, 2), (3, 3), (4, 5), (5, 8)];
        for (input, expect) in testcases {
            let result = Solution::climb_stairs(input);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    fn bench_climb_stairs(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::climb_stairs(39);
            assert_eq!(102334155, result);
        });
    }

    #[test]
    fn test_climb_stairs_v2() {
        let testcases: Vec<(i32, i32)> = vec![(1, 1), (2, 2), (3, 3), (4, 5), (5, 8)];
        for (input, expect) in testcases {
            let result = Solution::climb_stairs_v2(input);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    fn bench_climb_stairs_v2(b: &mut Bencher) {
        b.iter(|| {
            let result = Solution::climb_stairs_v2(39);
            assert_eq!(102334155, result);
        });
    }
}
