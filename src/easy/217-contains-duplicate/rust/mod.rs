use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<i32> = HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            } else {
                set.insert(num);
            }
        }
        false
    }
}

fn main () {
    let nums = vec![1, 2, 3, 1];
    let result = Solution::contains_duplicate(nums);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert!(!Solution::contains_duplicate(vec![8, 2, 3, 1]));
        assert!(!Solution::contains_duplicate(vec![55, 12, 42, 45]));
        assert!(Solution::contains_duplicate(vec![12, 15, 16, 32, 35, 42, 3, 2, 4, 2]));
    }
}