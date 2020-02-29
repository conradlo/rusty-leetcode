> note: runtime are weird and just for reference

|  ID  |                           Title                            | Difficulty | Runtime | Memory Usage | Remarks | Last Review |
| :--: | :--------------------------------------------------------: | :--------: | :-----: | :----------: | :-----: | :---------: |
| 0001 |                          Two Sum                           |    Easy    |  0 ms   |    2.4 MB    |         |             |
| 0020 |                     Valid Parentheses                      |    Easy    |  0 ms   |    2.1 MB    |         |             |
| 0021 |                   Merge Two Sorted Lists                   |    Easy    |  0 ms   |    2.0 MB    |         |             |
| 0024 |                    Swap Nodes in Pairs                     |   Medium   |  0 ms   |    2.0 MB    |         |             |
| 0070 |                      Climbing Stairs                       |    Easy    |  0 ms   |    2.1 MB    |         |             |
| 0094 |               Binary Tree Inorder Traversal                |   Medium   |  0 ms   |    2.0 MB    |         |             |
| 0098 |                Validate Binary Search Tree                 |   Medium   |  0 ms   |    2.8 MB    |         |             |
| 0101 |                       Symmetric Tree                       |    Easy    |  0 ms   |    2.2 MB    |   ⭐    |             |
| 0102 |             Binary Tree Level Order Traversal              |   Medium   |  0 ms   |    2.2 MB    |         |             |
| 0104 |                Maximum Depth of Binary Tree                |    Easy    |  0 ms   |    2.4 MB    |         | 2020/02/24  |
| 0105 | Construct Binary Tree from Preorder and Inorder Traversal  |   Medium   |  8 ms   |   37.9 MB    |   🔥    |             |
| 0106 | Construct Binary Tree from Inorder and Postorder Traversal |   Medium   |  4 ms   |   37.9 MB    |   🔥    |             |
| 0112 |                          Path Sum                          |    Easy    |  0 ms   |    2.5 MB    |         |             |
| 0119 |                    Pascal's Triangle II                    |    Easy    |  0 ms   |     2 MB     |         |             |
| 0144 |               Binary Tree Preorder Traversal               |   Medium   |  0 ms   |    2.1 MB    |         |             |
| 0145 |              Binary Tree Postorder Traversal               |    Hard    |  0 ms   |    2.1 MB    |         |             |
| 0150 |              Evaluate Reverse Polish Notation              |   Medium   |  0 ms   |    2.6 MB    |         |             |
| 0155 |                         Min Stack                          |    Easy    |  8 ms   |    5.5 MB    |         |             |
| 0200 |                     Number of Islands                      |   Medium   |  4 ms   |    5.5 MB    |         |             |
| 0206 |                    Reverse Linked List                     |    Easy    |  0 ms   |    2.3 MB    |         |             |
| 0279 |                      Perfect Squares                       |   Medium   |  24 ms  |    2.2 MB    |   🔥    |             |
| 0344 |                       Reverse String                       |    Easy    |  16 ms  |    5.3 MB    |         |             |
| 0394 |                       Decode String                        |   Medium   |  0 ms   |    1.9 MB    |   ⭐    |             |
| 0494 |                         Target Sum                         |   Medium   |  8 ms   |    2.2 MB    |  ⭐🔥⏲  | 2020/02/19  |
| 0509 |                      Fibonacci Number                      |    Easy    |  0 ms   |    1.9 MB    |         |             |
| 0542 |                         01 Matrix                          |   Medium   |  32 ms  |    3.1 MB    |   🔥    |             |
| 0622 |                   Design Circular Queue                    |   Medium   |  4 ms   |    2.2 MB    |         |             |
| 0700 |               Search in a Binary Search Tree               |    Easy    |  4 ms   |    2.6 MB    |         |             |
| 0733 |                         Flood Fill                         |    Easy    |  0 ms   |    1.9 MB    |         |             |
| 0739 |                     Daily Temperatures                     |   Medium   |  20 ms  |    2.8 MB    |         |             |
| 0752 |                       Open the Lock                        |   Medium   | 108 ms  |    3.5 MB    |   🔥    |             |
| 0779 |                   K-th Symbol in Grammar                   |   Medium   |  0 ms   |    2.1 MB    |   ⭐    |             |
| 0836 |                     Rectangle Overlap                      |    Easy    |  0 ms   |    2.0 MB    |         |             |
| 0841 |                       Keys and Rooms                       |   Medium   |  0 ms   |    2.1 MB    |         |             |
| 0912 |                       Sort an Array                        |   Medium   |  20 ms  |    2.9 MB    |   🔥    |             |  |

---

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
   use std::collections::HashMap; // <- put import(s) here
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
