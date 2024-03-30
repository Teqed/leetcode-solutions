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
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, mut k: i32) -> i32 {
        let mut distinct_count = vec![0; nums.len() + 1];
        let mut total_count = 0;
        let mut left = 0;
        let mut curr_count = 0;
        for right in 0..nums.len() {
            distinct_count[nums[right] as usize] += 1;
            if distinct_count[nums[right] as usize] == 1 {
                k -= 1;
            }
            if k < 0 {
                while k < 0 {
                    distinct_count[nums[left] as usize] -= 1;
                    if distinct_count[nums[left] as usize] == 0 {
                        k += 1;
                    }
                    left += 1;
                    curr_count = 0;
                }
            }
            if k == 0 {
                while distinct_count[nums[left] as usize] > 1 {
                    distinct_count[nums[left] as usize] -= 1;
                    left += 1;
                    curr_count += 1;
                }
                total_count += curr_count + 1;
            }
        }
        total_count
    }
}
fn main() {
    let nums = vec![1, 2, 1, 2, 3];
    let k = 2;
    let result = Solution::subarrays_with_k_distinct(nums, k);
    println!("Result: {}", result);
    assert!(result == 7)
}