/*
 * @lc app=leetcode id=841 lang=rust
 *
 * [841] Keys and Rooms
 */

struct Solution;
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        // traverse an adjacency list
        let no_of_rooms = rooms.len();
        if no_of_rooms == 1 {
            return true;
        }
        let mut visited = vec![0; no_of_rooms];
        let mut room_count = 0;

        let mut q: VecDeque<i32> = VecDeque::new();
        q.push_back(0);
        visited[0] = 1;
        while let Some(fst_room) = q.pop_front() {
            let room_id = fst_room as usize;
            room_count += 1;
            // println!("{}| {}", room_count, room_id);
            let keys = &rooms[room_id];
            for key in keys {
                let next_room = *key;
                let next_room_id = next_room as usize;
                if visited[next_room_id] == 0 {
                    q.push_back(next_room);
                    visited[next_room_id] = 1;
                }
            }
        }
        return room_count == no_of_rooms;
    }
}
// @lc code=end

// cargo watch -x "test _0841_"
extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use test::Bencher;

    #[test]
    pub fn test_can_visit_all_rooms() {
        let testcases: Vec<(Vec<Vec<i32>>, bool)> = vec![
            (vec![vec![0]], true),
            (vec![vec![0], vec![0]], false),
            (vec![vec![1], vec![0]], true),
            (vec![vec![1], vec![2], vec![3], vec![]], true),
            (vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]], false),
        ];
        for (testcase, expect) in testcases {
            // println!("{:?} {}", testcase.clone(), expect);
            let result = Solution::can_visit_all_rooms(testcase);
            assert_eq!(result, expect);
        }
    }
    fn gen_large_connected_rooms() -> Vec<Vec<i32>> {
        let mut rand_rooms: Vec<Vec<i32>> = vec![];
        for i in 0..1000 {
            let mut next: Vec<i32> = vec![];
            for j in 1..4 {
                if i + j < 1000 {
                    next.push(i + j);
                }
            }
            rand_rooms.push(next);
        }
        return rand_rooms;
    }
    fn gen_large_disconnected_rooms() -> Vec<Vec<i32>> {
        let mut rng = rand::thread_rng();
        let mut rand_rooms: Vec<Vec<i32>> = vec![];
        for i in 0..1000 {
            // odd rooms only link to odd room
            // even rooms only link to even room
            let l = rng.gen_range(0, 20);
            let mut next: Vec<i32> = vec![];
            for _ in 0..l {
                let k = rng.gen_range(0, 1000);
                if i % 2 == 0 {
                    if k % 2 == 0 {
                        next.push(k);
                    } else {
                        next.push(k - 1);
                    }
                } else {
                    if k % 2 == 0 {
                        if k != 0 {
                            next.push(k - 1);
                        }
                    } else {
                        next.push(k);
                    }
                }
            }
            rand_rooms.push(next);
        }
        return rand_rooms;
    }

    #[bench]
    pub fn bench_can_visit_all_rooms(b: &mut Bencher) {
        // let rooms = gen_large_disconnected_rooms();
        let rooms = gen_large_connected_rooms();
        b.iter(|| {
            let result = Solution::can_visit_all_rooms(rooms.clone());
            // assert_eq!(result, false);
            assert_eq!(result, true);
        });
    }
}
