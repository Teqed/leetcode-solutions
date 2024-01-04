use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![1, 0];
        }
        let mut number_collection = HashSet::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if number_collection.contains(&complement) {
                return vec![
                    i as i32,
                    nums.iter().position(|&x| x == complement).unwrap() as i32,
                ];
            }
            number_collection.insert(num);
        }
        vec![]
    }
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{}, {}", result[0], result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![2, 1]);
    }

    #[test]
    fn test_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![1, 0]);
    }
}
