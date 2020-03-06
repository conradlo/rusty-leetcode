/*
 * @lc app=leetcode id=1266 lang=rust
 *
 * [1266] Minimum Time Visiting All Points
 */

struct Solution;
// @lc code=start
// use std::cmp::Ordering;
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        if points.len() < 2 {
            return ans;
        }
        // loop thru each adjacent point pairs
        for i in 1..points.len() {
            let a = &points[i - 1];
            let b = &points[i];

            // let say if we can only go (left or right) or (up or down)
            // then the min time will be x_abs + y_abs
            // now we can go diagonally
            // we would prefer to go as much diagonally as possible
            // (becoz we can traval sqrt(2) > 1, in same time (1s))
            // so whenever there is (1 left or right, 1 up or down) pair
            // we can go diagonally
            let x_abs_diff = i32::abs(a[0] - b[0]); // no. of left/right
            let y_abs_diff = i32::abs(a[1] - b[1]); // no. of up/down

            /*
            ans += match x_abs_diff.cmp(&y_abs_diff) {
              Ordering::Less => x_abs_diff + (y_abs_diff - x_abs_diff), // no. of diagonals + vertical remains
              Ordering::Equal => x_abs_diff, // no. of diagonals == x_abs_diff || y_abs_diff
              Ordering::Greater => y_abs_diff + (x_abs_diff - y_abs_diff), // no. of diagonals + horizontals remains
            }
            */

            // simplify login above, add whichever larger, the same thing
            ans += if x_abs_diff < y_abs_diff {
                y_abs_diff
            } else {
                x_abs_diff
            }
        }
        ans
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_time_to_visit_all_points() {
        let testcases = vec![
            (vec![vec![1, 1]], 0),
            (vec![vec![1, 1], vec![3, 4], vec![-1, 0]], 7),
            (vec![vec![3, 2], vec![-2, 2]], 5),
            (
                vec![
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 3],
                    vec![4, 4],
                    vec![5, 5],
                    vec![-1, 1],
                    vec![-2, 2],
                    vec![-3, 3],
                    vec![-4, -4],
                    vec![-100, 100],
                ],
                123,
            ),
        ];

        for (input, expect) in testcases {
            let result = Solution::min_time_to_visit_all_points(input);
            assert_eq!(result, expect);
        }
    }
}
