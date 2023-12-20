use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0, 1];
        }
        let mut set = HashSet::new();
        for (i, num) in nums.iter().enumerate() {
            let diff = target - num;
            if set.contains(&diff) {
                return vec![i as i32, nums.iter().position(|&x| x == diff).unwrap() as i32];
            }
            set.insert(num);
        }
        vec![] // Return an empty vector for the case when the loop has zero elements to iterate on
    }
}

fn main () {
    let nums = vec![2,7,11,15];
    let target = 9;
    let result = Solution::two_sum(nums, target);
    println!("{}, {}", result[0], result[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert!(!Solution::two_sum(vec![2,7,11,15], 9));
        assert!(!Solution::two_sum(vec![3,2,4], 6));
        assert!(!Solution::two_sum(vec![3,3], 6));
    }
}