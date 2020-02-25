use std::collections::{HashMap, HashSet, VecDeque};
pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
    let mut visited: HashSet<String> = HashSet::new();
    // treat all deadends as visited nodes
    for val in deadends {
        visited.insert(val);
    }

    let start = String::from("0000");
    if visited.contains(&start) {
        return -1;
    }
    // save time on char <-> digit conversion
    let digit_list: [i32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let digit_char_list: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let mut char_to_digit_map: HashMap<char, i32> = HashMap::new();
    let mut digit_to_char_map: HashMap<i32, char> = HashMap::new();
    for i in 0..10 {
        char_to_digit_map.insert(digit_char_list[i], digit_list[i]);
        digit_to_char_map.insert(digit_list[i], digit_char_list[i]);
    }

    let mut queue: VecDeque<(usize, String)> = VecDeque::new();
    queue.push_back((0, start));
    while !queue.is_empty() {
        let (level, head) = queue.pop_front().unwrap();
        if visited.contains(&head) {
            continue;
        } else {
            visited.insert(head.clone());
        }
        if head == target {
            return level as i32;
        } else {
            // BFS on 'head'
            // create 8 reachable nodes from 'head'
            let mut next: [[char; 4]; 8] = [['0'; 4]; 8];
            for (i, c) in head.char_indices() {
                let n = char_to_digit_map.get(&c).unwrap();
                for (j, combo) in next.iter_mut().enumerate().take(4) {
                    combo[i] = if j == i {
                        let k = (n + 1) % 10;
                        *digit_to_char_map.get(&k).unwrap()
                    } else {
                        c
                    }
                }
                for j in 0..4 {
                    next[j + 4][i] = if j == i {
                        let k = (n - 1 + 10) % 10;
                        *digit_to_char_map.get(&k).unwrap()
                    } else {
                        c
                    }
                }
            }

            // push all 8 nodes onto the queue
            for node in &next {
                // convert [char] to String
                let key = node.iter().collect::<String>();
                queue.push_back((level + 1, key));
            }
        }
    }
    -1
}

// cargo watch -x "test _0752_ -- --nocapture"
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run_test_cases() {
        let test_cases = vec![
            ((vec!["0201", "0101", "0102", "1212", "2002"], "0202"), 6),
            ((vec!["8888"], "0009"), 1),
            (
                (
                    vec![
                        "8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888",
                    ],
                    "8888",
                ),
                -1,
            ),
            ((vec!["0000"], "8888"), -1),
        ];
        for (input, expect) in test_cases {
            let (deadends, target) = input;
            let ans = open_lock(
                deadends.iter().map(|&val| String::from(val)).collect(),
                String::from(target),
            );
            assert_eq!(ans, expect);
        }
    }
}
