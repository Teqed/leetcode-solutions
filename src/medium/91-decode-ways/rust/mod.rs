struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = if s.chars().nth(0).unwrap() == '0' { 0 } else { 1 };
        for i in 2..=s.len() {
            let one = s[i - 1..i].parse::<i32>().unwrap();
            let two = s[i - 2..i].parse::<i32>().unwrap();
            if (1..=9).contains(&one) {
                dp[i] += dp[i - 1];
            }
            if (10..=26).contains(&two) {
                dp[i] += dp[i - 2];
            }
        }
        dp[s.len()]
    }
}

fn main() {
    let input = "12".to_string();
    println!("Input: {:?}", input);
    println!("Output: {:?}", Solution::num_decodings(input));
    let input2 = "226".to_string();
    println!("Input: {:?}", input2);
    println!("Output: {:?}", Solution::num_decodings(input2));
    let input3 = "0".to_string();
    println!("Input: {:?}", input3);
    println!("Output: {:?}", Solution::num_decodings(input3));
    let input4 = "06".to_string();
    println!("Input: {:?}", input4);
    println!("Output: {:?}", Solution::num_decodings(input4));
}