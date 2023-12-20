struct Solution;

// use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // General solution for all Unicode characters:
        // let mut counter = HashMap::new();
        // s.chars().for_each(|c| *counter.entry(c).or_insert(0) += 1);
        // for c in t.chars() {
        //     let count = counter.entry(c).or_insert(0);
        //     *count -= 1;
        //     if *count < 0 {
        //         return false;
        //     }
        // }
        // counter.values().all(|&count| count == 0)

        // Constraint: s and t consist of lowercase English letters:
        if s.len() != t.len() {
            return false;
        }
        let mut counter = [0; 26];
        for c in s.chars() {
            counter[c as usize - 'a' as usize] += 1;
        }
        for c in t.chars() {
            counter[c as usize - 'a' as usize] -= 1;
            if counter[c as usize - 'a' as usize] < 0 {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "is_anagram('anagram', 'nagaram')={}",
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string())
    );
    println!(
        "is_anagram('rat', 'car')={}",
        Solution::is_anagram("rat".to_string(), "car".to_string())
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_242() {
        assert!(Solution::is_anagram(
            "anagram".to_string(),
            "nagaram".to_string()
        ));
        assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    }
}