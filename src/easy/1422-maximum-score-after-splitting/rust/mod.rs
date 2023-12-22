// [1422\. Maximum Score After Splitting a String](https://leetcode.com/problems/maximum-score-after-splitting-a-string/)

// Easy

// Topics

// Companies

// Hint

// Given a string `s` of zeros and ones, *return the maximum score after splitting the string into two non-empty substrings* (i.e. left substring and right substring).

// The score after splitting a string is the number of zeros in the left substring plus the number of ones in the right substring.

// Example 1:

// Input: s = "011101"
// Output: 5
// Explanation:
// All possible ways of splitting s into two non-empty substrings are:
// left = "0" and right = "11101", score = 1 + 4 = 5
// left = "01" and right = "1101", score = 1 + 3 = 4
// left = "011" and right = "101", score = 1 + 2 = 3
// left = "0111" and right = "01", score = 1 + 1 = 2
// left = "01110" and right = "1", score = 2 + 1 = 3

// Example 2:

// Input: s = "00111"
// Output: 5
// Explanation: When left = "00" and right = "111", we get the maximum score = 2 + 3 = 5

// Example 3:

// Input: s = "1111"
// Output: 3

// Constraints:

// -   `2 <= s.length <= 500`
// -   The string `s` consists of characters `'0'` and `'1'` only.

struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let (mut result, mut zeros, mut ones) = (
            0, 0, s.chars().filter(|&character| character == '1').count() as i32);
        for (_i, character) in s.chars().enumerate().take(s.len() - 1) {
            match character {
                '0' => zeros += 1,
                '1' => ones -= 1,
                _ => {}
            }
            result = result.max(zeros + ones);
        }
        result
    }
}

fn main() {
    let input = "011101".to_string();
    println!("Input: {:?}", input);
    println!("Output: {:?}", Solution::max_score(input));
    let input2 = "00111".to_string();
    println!("Input: {:?}", input2);
    println!("Output: {:?}", Solution::max_score(input2));
    let input3 = "1111".to_string();
    println!("Input: {:?}", input3);
    println!("Output: {:?}", Solution::max_score(input3));
    let input4 = "01001".to_string();
    println!("Input: {:?}", input4);
    println!("Output: {:?}", Solution::max_score(input4));
}
