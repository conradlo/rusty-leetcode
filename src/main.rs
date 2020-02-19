#![feature(test)] // for `cargo bench`
#![allow(dead_code)] // allow dead code at crate-level

// import code in main.rs such that `cargo test` will run all tests inside these modules
mod _0001_two_sum;
mod _0020_valid_parentheses;
mod _0024_swap_nodes_in_pairs;
mod _0094_binary_tree_inorder_traversal;
mod _0101_symmetric_tree;
mod _0102_binary_tree_level_order_traversal;
mod _0104_maximum_depth_of_binary_tree;
mod _0106_construct_binary_tree_from_inorder_and_postorder_traversal;
mod _0112_path_sum;
mod _0144_binary_tree_preorder_traversal;
mod _0145_binary_tree_postorder_traversal;
mod _0150_evaluate_reverse_polish_notation;
mod _0155_min_stack;
mod _0200_number_of_islands;
mod _0206_reverse_linked_list;
mod _0279_perfect_squares;
mod _0344_reverse_string;
mod _0394_decode_string;
mod _0494_target_sum;
mod _0622_design_circular_queue;
mod _0700_search_in_a_binary_search_tree;
mod _0733_flood_fill;
mod _0739_daily_temperatures;
mod _0752_open_the_lock;
mod other;

fn main() {
    println!("Hello leetcode!");
}
