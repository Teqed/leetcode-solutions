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
