pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            let a = nums[i];
            let b = nums[j];
            if a + b == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

use std::collections::HashMap;

pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rmb = HashMap::<i32, i32>::new(); // val, idx
    let mut complements = Vec::<i32>::new();
    // destructure a tuple of (usize, &i32)
    for (i, &num) in nums.iter().enumerate() {
        complements.push(target - num);
        rmb.insert(num, i as i32);
    }
    for (i, com) in complements.iter().enumerate() {
        if let Some(&j) = rmb.get(com) {
            // ! important
            if j != i as i32 {
                return vec![i as i32, j];
            }
        }
    }
    return vec![];
}

// best ans [0ms, 2.4md]
pub fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() > 2 {
        let mut dict = HashMap::<i32, i32>::new();
        for (i, &num) in nums.iter().enumerate() {
            // use option destructuring can achieve 4ms
            // if let Some(&j) = dict.get(num) {
            //     return vec![j, i as i32];
            // }

            // use `contains_key` can achieve 0ms
            if dict.contains_key(&num) {
                return vec![dict[&num], i as i32];
            }
            dict.insert(target - num, i as i32);
        }
    }
    return vec![0, 1];
}
// cargo watch -x "test _0001_ -- --nocapture"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    type Input = (Vec<i32>, i32);
    type Expect = [i32; 2];

    fn get_test_cases() -> Vec<(Input, Expect)> {
        vec![
            ((vec![2, 7, 11, 15], 9), [0, 1]),
            ((vec![4, 3, 1, 2], 3), [2, 3]),
            ((vec![1, 0, 0, -1, 2], 3), [0, 4]),
            ((vec![-1, 0, 4, 1, -2], 2), [2, 4]),
            ((vec![3, 2, 4], 6), [1, 2]),
        ]
    }

    #[test]
    fn test_v1() {
        let test_cases = get_test_cases();
        for ((nums, target), expect) in test_cases {
            let ans = two_sum_1(nums, target);
            assert_eq!(ans, expect);
        }
    }

    #[bench]
    fn bench_v1(b: &mut Bencher) {
        // let mut rng = rand::thread_rng();
        let mut nums = vec![0; 1_000];
        nums[999] = -1;
        nums[998] = -1;

        b.iter(|| {
            let result = two_sum_1(nums.clone(), -2);
            assert_eq!(result, [998, 999]);
        });
    }

    #[test]
    fn test_v2() {
        let test_cases = get_test_cases();
        for ((nums, target), expect) in test_cases {
            let ans = two_sum_2(nums, target);
            assert_eq!(ans, expect);
        }
    }

    #[bench]
    fn bench_v2(b: &mut Bencher) {
        // let mut rng = rand::thread_rng();
        let mut nums = vec![0; 1_000];
        nums[999] = -1;
        nums[998] = -1;

        b.iter(|| {
            let result = two_sum_2(nums.clone(), -2);
            assert_eq!(result, [998, 999]);
        });
    }

    #[test]
    fn test_v3() {
        let test_cases = get_test_cases();
        for ((nums, target), expect) in test_cases {
            let ans = two_sum_3(nums, target);
            assert_eq!(ans, expect);
        }
    }

    #[bench]
    fn bench_v3(b: &mut Bencher) {
        // let mut rng = rand::thread_rng();
        let mut nums = vec![0; 1_000];
        nums[999] = -1;
        nums[998] = -1;

        b.iter(|| {
            let result = two_sum_3(nums.clone(), -2);
            assert_eq!(result, [998, 999]);
        });
    }
}
