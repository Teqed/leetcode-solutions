struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut frequency = std::collections::HashMap::new();
        for num in nums {
            *frequency.entry(num).or_insert(0) += 1;
        }

        let mut buckets = vec![vec![]; frequency.len() + 2];
        for (num, freq) in frequency {
            buckets[freq].push(num);
        }

        let mut result = vec![];
        for bucket in buckets.into_iter().rev() {
            for num in bucket {
                result.push(num);
                if result.len() == k as usize {
                    return result;
                }
            }
        }
        
        result
    }
}

fn main() {
    let input1a = vec![1, 1, 1, 2, 2, 3];
    let input1b = 2;
    println!("Input: nums = {:?}, k = {}", input1a, input1b);
    println!("Output: {:?}", Solution::top_k_frequent(input1a, input1b));
    let input2a = vec![1];
    let input2b = 1;
    println!("Input: nums = {:?}, k = {}", input2a, input2b);
    println!("Output: {:?}", Solution::top_k_frequent(input2a, input2b));
    let input3a = vec![-1, -1];
    let input3b = 1;
    println!("Input: nums = {:?}, k = {}", input3a, input3b);
    println!("Output: {:?}", Solution::top_k_frequent(input3a, input3b));
}
