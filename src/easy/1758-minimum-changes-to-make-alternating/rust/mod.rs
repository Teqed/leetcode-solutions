struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let (a, b) = s.chars().enumerate().fold((0, 0), |(a, b), (i, c)| {
            if c == (if i % 2 == 0 { '0' } else { '1' }) { (a + 1, b) } else { (a, b + 1) }
        });
        std::cmp::min(a, b)
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