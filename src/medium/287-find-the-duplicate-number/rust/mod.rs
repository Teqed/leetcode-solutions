#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::question_mark_used
)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        let length_of_nums = nums.len();
        let mut i = 1;
        for number in nums {
            if i == length_of_nums {
                return number;
            }
            if !seen.insert(number) {
                return number;
            }
            i += 1;
        }
        0
    }
}

fn main() {
    let input1 = vec![1, 3, 4, 2, 2];
    println!("Input: {:?}", input1);
    println!("Output: {}", Solution::find_duplicate(input1));
    let input2 = vec![3, 1, 3, 4, 2];
    println!("Input: {:?}", input2);
    println!("Output: {}", Solution::find_duplicate(input2));
    let input3 = vec![1, 1];
    println!("Input: {:?}", input3);
    println!("Output: {}", Solution::find_duplicate(input3));
    let input4 = vec![1, 1, 2];
    println!("Input: {:?}", input4);
    println!("Output: {}", Solution::find_duplicate(input4));
}