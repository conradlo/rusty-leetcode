
# Submit solution via command line
1. install `leetcode-cli`
    - https://github.com/skygragon/leetcode-cli
1. login LeetCode via `leetcode-cli`
    - Cannot login in LeetCode via leetcode-cli? https://github.com/jdneo/vscode-leetcode/issues/478#issuecomment-564757098k
1. Fetch LeetCode problem by Problem ID (note: do not track in these files)
    ```sh
    # leetcode show {problem_id} -g -l {language}
    leetcode show 1 -g -l rust
    ```
1. Copy code from /src
    - all code should place inside `// @lc code=start` and `// @lc code=end`
    ```rust
    // @lc code=start
    use std::collections::HashMap; // <- put import here
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // ...
        }
    }
    // @lc code=end
    ```
1. LeetCode test from command line
    ```sh
    # leetcode test {file_name}.rs
    leetcode test 1.two-sum.rs

    # with test case
    leetcode test 1.two-sum.rs -t "[-1, 0, 4, 1, -2]\n2"
    ```
1. Submit and get result
    ```sh
    # leetcode submit {file_name}.rs
    leetcode submit 1.two-sum.rs
    ```

# Reference
https://zhuanlan.zhihu.com/p/33211817
