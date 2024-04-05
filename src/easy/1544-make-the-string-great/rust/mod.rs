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
    pub fn make_good(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.chars() {
            if stack.is_empty() {
                stack.push(c);
            } else {
                let top = stack.last().unwrap();
                if (*top as i32 - c as i32).abs() == 32 {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
        }
        stack.iter().collect()
    }
}

fn main() {
    let s = "leEeetcode".to_string();
    let result = Solution::make_good(s);
    println!("Result: {}", result);
    assert!(result == "leetcode".to_string())
}