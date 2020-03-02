/*
 * @lc app=leetcode id=599 lang=rust
 *
 * [599] Minimum Index Sum of Two Lists
 */

struct Solution;
// @lc code=start
use std::cmp::Ordering;
use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant_v1(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        // find all matches
        // n * n
        let mut matches: Vec<(usize, String)> = vec![];
        for (i, res1) in list1.iter().enumerate() {
            //
            for (j, res2) in list2.iter().enumerate() {
                if res1 == res2 {
                    matches.push((i + j, res2.clone()));
                }
            }
        }

        // sort matches by index sum
        matches.sort_by_key(|k| k.0);
        let mut ans = vec![];

        // only rmb matches with min index sum
        let mut min_idx_sum = 0;
        for (sum, res) in matches {
            if ans.is_empty() {
                min_idx_sum = sum;
                ans.push(res);
            } else if sum == min_idx_sum {
                ans.push(res);
            } else {
                break;
            }
        }

        // return all matches with min index sum
        ans
    }

    // use hash map as lookup table
    // Time: time to build HashMap, and look up
    // O(|N1| + |N2|)
    // Space: size of HashMap O(n) + ans list O(n)
    pub fn find_restaurant_v2(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let table_list = if list1.len() < list2.len() {
            &list1
        } else {
            &list2
        };
        let search_list = if list1.len() < list2.len() {
            &list2
        } else {
            &list1
        };
        let mut table: HashMap<String, usize> = HashMap::with_capacity(table_list.len());
        let mut ans: Vec<String> = vec![];
        let mut min_index_sum: usize = std::usize::MAX;
        // build lookup table
        for (i, item) in table_list.iter().enumerate() {
            table.insert(item.clone(), i);
        }
        // lookup and rmb min index sum's restaurant names
        for (j, key) in search_list.iter().enumerate() {
            if let Some(i) = table.get(key) {
                let index_sum = j + i;
                match index_sum.cmp(&min_index_sum) {
                    Ordering::Less => {
                        // new min, refresh the list
                        min_index_sum = index_sum;
                        ans = vec![key.clone()];
                    }
                    Ordering::Equal => {
                        // old min, push to the list
                        ans.push(key.clone());
                    }
                    Ordering::Greater => (),
                };
            }
        }

        ans
    }

    // use own hash table?
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    type Input = (Vec<String>, Vec<String>);
    type Expect = Vec<String>;
    fn get_test_cases() -> Vec<(Input, Expect)> {
        let testcases = vec![
            (
                (
                    vec!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                    vec![
                        "Piatti",
                        "The Grill at Torrey Pines",
                        "Hungry Hunter Steakhouse",
                        "Shogun",
                    ],
                ),
                vec!["Shogun"],
            ),
            (
                (
                    vec!["Shogun", "Tapioca Express", "Burger King", "KFC"],
                    vec!["KFC", "Shogun", "Burger King"],
                ),
                vec!["Shogun"],
            ),
            (
                (
                    vec!["Shogun", "Tapioca Express", "B", "KFC"],
                    vec!["Tapioca Express", "Shogun", "A", "KFC"],
                ),
                vec!["Shogun", "Tapioca Express"],
            ),
            (
                (vec!["AA", "C", "B", "KFC"], vec!["BB", "CC", "A", "KFC"]),
                vec!["KFC"],
            ),
        ];

        let mut v = vec![];
        for ((v1, v2), expect) in testcases {
            let v1 = v1.iter().map(|&s| String::from(s)).collect();
            let v2 = v2.iter().map(|&s| String::from(s)).collect();
            let expect = expect.iter().map(|&s| String::from(s)).collect();
            v.push(((v1, v2), expect));
        }
        v
    }

    #[test]
    fn test_v1() {
        for ((v1, v2), expect) in get_test_cases() {
            let mut set_expect = HashSet::new();
            for s in expect {
                set_expect.insert(s);
            }
            let mut set_result = HashSet::new();
            for s in Solution::find_restaurant_v1(v1, v2) {
                set_result.insert(s);
            }
            // let result = Solution::find_restaurant_v1(v1, v2);
            assert_eq!(set_result, set_expect);
        }
    }

    #[test]
    fn test_v2() {
        for ((v1, v2), expect) in get_test_cases() {
            let mut set_expect = HashSet::new();
            for s in expect {
                set_expect.insert(s);
            }
            let mut set_result = HashSet::new();
            for s in Solution::find_restaurant_v2(v1, v2) {
                set_result.insert(s);
            }
            // let result = Solution::find_restaurant_v1(v1, v2);
            assert_eq!(set_result, set_expect);
        }
    }
}
