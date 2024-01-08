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

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut hash = std::collections::HashMap::new();
        for num in nums {
            hash.insert(num, true);
        }
        let mut longest_sequence = 0;
        for num in hash.keys() {
            if !hash.contains_key(&(num - 1)) {
                let mut sequence = 1;
                while hash.contains_key(&(num + sequence)) {
                    sequence += 1;
                }
                if sequence > longest_sequence {
                    longest_sequence = sequence;
                }
            }
        }
        longest_sequence
    }
}

fn main () {
    let test_cases = vec![
        vec![100,4,200,1,3,2], // 4
        vec![0,3,7,2,5,8,4,6,0,1], // 9
    ];
    for test_case in test_cases {
        println!("{}", Solution::longest_consecutive(test_case));
    }
}