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
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut jobs: Vec<usize> = (0..start_time.len()).collect();
        jobs.sort_by(|a, b| end_time[*a].cmp(&end_time[*b]));
        let mut v = vec![(0, 0)];
        for i in jobs {
            let mut left = 0;
            let mut right = v.len() - 1;
            while left <= right {
                let mid = (left + right) / 2;
                if v[mid].1 <= start_time[i] {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            let mut max_profit = v[right].0 + profit[i];
            if max_profit < v[v.len() - 1].0 {
                max_profit = v[v.len() - 1].0;
            }
            v.push((max_profit, end_time[i]));
        }
        v[v.len() - 1].0
    }
}

fn main() {
    // Input 0
    let start_time = vec![1, 2, 3, 3];
    let end_time = vec![3, 4, 5, 6];
    let profit = vec![50, 10, 40, 70];
    // Output 0 120
    println!(
        "Output 0 {}",
        Solution::job_scheduling(start_time, end_time, profit)
    );
    // Input 1
    let start_time = vec![1, 2, 3, 4, 6];
    let end_time = vec![3, 5, 10, 6, 9];
    let profit = vec![20, 20, 100, 70, 60];
    // Output 1 150
    println!(
        "Output 1 {}",
        Solution::job_scheduling(start_time, end_time, profit)
    );
    // Input 2
    let start_time = vec![1, 1, 1];
    let end_time = vec![2, 3, 4];
    let profit = vec![5, 6, 4];
    // Output 2 6
    println!(
        "Output 2 {}",
        Solution::job_scheduling(start_time, end_time, profit)
    );
}
