struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let count = s.chars().enumerate().fold(0, |a, (index, character)| {
            if character == (if index % 2 == 0 { '0' } else { '1' }) {
                a + 1
            } else {
                a
            }
        });
        std::cmp::min(count, s.len() as i32 - count)
    }
}

fn main() {
    let input = "0100".to_string();
    println!("Input: {:?}", input);
    println!("Output: {:?}", Solution::min_operations(input));
    let input2 = "10".to_string();
    println!("Input: {:?}", input2);
    println!("Output: {:?}", Solution::min_operations(input2));
    let input3 = "1111".to_string();
    println!("Input: {:?}", input3);
    println!("Output: {:?}", Solution::min_operations(input3));
}
