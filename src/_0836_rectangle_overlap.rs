/*
 * @lc app=leetcode id=836 lang=rust
 *
 * [836] Rectangle Overlap
 */

struct Solution;
// @lc code=start
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        // [x1, y1, x2, y2]
        // [0   1   2   3]
        !(rec1[3] <= rec2[1] || rec1[1] >= rec2[3] || rec1[0] >= rec2[2] || rec1[2] <= rec2[0])
    }
}
// @lc code=end

// cargo watch -x "test _0836_"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1() {
        let testcases = vec![
            ((vec![0, 0, 1, 1], vec![0, 0, 1, 1]), true),
            ((vec![0, 0, 1, 1], vec![1, 0, 2, 1]), false),
            ((vec![0, 0, 1, 1], vec![0, 1, 1, 2]), false),
            ((vec![0, 0, 1, 1], vec![0, -1, 1, 0]), false),
            ((vec![0, 0, 1, 1], vec![-1, 0, 0, 1]), false),
            ((vec![0, 0, 1, 1], vec![1, 1, 2, 2]), false),
            ((vec![0, 0, 2, 2], vec![1, 1, 2, 2]), true),
            ((vec![0, 0, 2, 2], vec![1, 1, 3, 3]), true),
        ];

        for ((rec1, rec2), expect) in testcases {
            let result = Solution::is_rectangle_overlap(rec1, rec2);
            assert_eq!(result, expect);
        }
    }
}
