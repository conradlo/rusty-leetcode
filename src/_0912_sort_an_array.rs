/*
 * @lc app=leetcode id=912 lang=rust
 *
 * [912] Sort an Array
 */

struct Solution;

/*

How many sorting algorithm do you know?

Bubble Sort
Insertion Sort
Selection Sort

Quick Sort
Merge Sort
Heap Sort

Counting Sort
Radix sort
Bucket Sort

*/

// 1 <= nums.length <= 50000
// -50000 <= nums[i] <= 50000
// @lc code=start
use std::cmp::Ordering;
impl Solution {
    // Merge sort, top down recursive
    pub fn merge_sorted_vector_v1(left: &[i32], right: &[i32]) -> Vec<i32> {
        // println!("{:?} {:?}", left, right);
        let mut l = 0;
        let mut r = 0;

        let mut v = vec![];

        while l < left.len() || r < right.len() {
            match (l.cmp(&left.len()), r.cmp(&right.len())) {
                (Ordering::Less, Ordering::Less) => {
                    if left[l] <= right[r] {
                        v.push(left[l]);
                        l += 1;
                    } else {
                        v.push(right[r]);
                        r += 1;
                    }
                }
                (Ordering::Less, Ordering::Equal) => {
                    v.push(left[l]);
                    l += 1;
                }
                (Ordering::Equal, Ordering::Less) => {
                    v.push(right[r]);
                    r += 1;
                }
                (_, _) => {}
            }
        }
        v
    }

    pub fn merge_sort_top_down_recursive(nums: Vec<i32>) -> Vec<i32> {
        // base case
        if nums.len() < 2 {
            return nums;
        }
        // divide & recur
        let mid = nums.len() / 2;
        let l_half = Solution::merge_sort_top_down_recursive(nums[..mid].to_vec());
        let r_half = Solution::merge_sort_top_down_recursive(nums[mid..].to_vec());

        // merge
        Solution::merge_sorted_vector_v1(&l_half, &r_half)
    }

    // quick sort
    // worst case of quick sort is just an insertion sort
    pub fn sort_array_quick_sort(nums: Vec<i32>) -> Vec<i32> {
        let length = nums.len();
        let mut nums = nums;
        // inplace
        Solution::quick_sort(&mut nums, 0, length - 1);
        nums
    }

    pub fn quick_sort(nums: &mut [i32], left: usize, right: usize) {
        // base case, no items / 1 items
        if left < right {
            let p = Solution::partition(nums, left, right);
            if p > 0 {
                // prevent subtract with overflow
                Solution::quick_sort(nums, left, p - 1);
            }
            Solution::quick_sort(nums, p + 1, right);
        }
    }

    pub fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
        let pivot = right; // use last item as pivot
        let mut left_bound = left;
        for i in left..right {
            if nums[i] < nums[pivot] {
                nums.swap(left_bound, i);
                left_bound += 1;
            }
        }
        // everything before 'left_bound' is smaller than nums[pivot]
        nums.swap(left_bound, right);
        left_bound
    }

    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        //
        nums
    }
}
// @lc code=end

// cargo watch -x "test _0912_ -- --nocapture --test-threads=1"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use test::Bencher;

    fn get_test_cases() -> Vec<(Vec<i32>, Vec<i32>)> {
        vec![
            (vec![5, 2, 3, 1], vec![1, 2, 3, 5]),
            (vec![5, 1, 1, 2, 0, 0], vec![0, 0, 1, 1, 2, 5]),
        ]
    }

    #[test]
    fn test_merge() {
        let testcases = vec![
            ((vec![], vec![]), vec![]),
            ((vec![], vec![1]), vec![1]),
            ((vec![1], vec![]), vec![1]),
            ((vec![2], vec![1]), vec![1, 2]),
            ((vec![1, 3, 5], vec![2, 4, 6]), vec![1, 2, 3, 4, 5, 6]),
        ];
        for ((left, right), expect) in testcases {
            let result = Solution::merge_sorted_vector_v1(&left, &right);
            assert_eq!(result, expect);
        }
    }

    #[test]
    fn test_merge_sort_sm() {
        for (input, expect) in get_test_cases() {
            let result = Solution::merge_sort_top_down_recursive(input);
            assert_eq!(result, expect);
        }
    }

    #[bench]
    fn test_merge_sort_lg(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_vector: Vec<i32> = vec![0; 50_000];
        for val in rand_vector.iter_mut() {
            *val = rng.gen_range(-50_000, 50_000);
        }
        let mut expect = rand_vector.clone();
        expect.sort_unstable();
        b.iter(|| {
            let result = Solution::merge_sort_top_down_recursive(rand_vector.clone());
            assert_eq!(result, expect);
        });
    }

    #[test]
    fn test_quick_sort() {
        for (input, expect) in get_test_cases() {
            let result = Solution::sort_array_quick_sort(input);
            assert_eq!(result, expect);
        }
    }
    #[bench]
    fn test_quick_sort_lg(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut rand_vector: Vec<i32> = vec![0; 50_000];
        for val in rand_vector.iter_mut() {
            *val = rng.gen_range(-50_000, 50_000);
        }
        let mut expect = rand_vector.clone();
        expect.sort_unstable();
        b.iter(|| {
            let result = Solution::sort_array_quick_sort(rand_vector.clone());
            assert_eq!(result, expect);
        });
    }
}
