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

struct Solution;

use std::{collections::HashMap, vec};

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut frequency = HashMap::new();
        for num in nums {
            let count = frequency.entry(num).or_insert(0);
            *count += 1;
        }
        let mut operations = 0;
        for (_num, count) in frequency {
            if count < 2 {
                return -1;
            }
            operations += (count + 2) / 3;
        }
        operations
    }
}

fn main() {
    // Input 0 [2,3,3,2,2,4,2,3,4]
    let nums = vec![2, 3, 3, 2, 2, 4, 2, 3, 4];
    // Output 0 4
    println!("Output 0 {}", Solution::min_operations(nums));
    // Input 1 [2,1,2,2,3,3]
    let nums = vec![2, 1, 2, 2, 3, 3];
    // Output 1 -1
    println!("Output 1 {}", Solution::min_operations(nums));
}
