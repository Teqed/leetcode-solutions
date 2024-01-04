struct Solution;

// impl Solution {
//     pub fn num_decodings(s: String) -> i32 {
//         let chars: Vec<_> = s.chars().collect();
//         chars.iter().zip(chars.iter().skip(1)).enumerate()
//             .fold([1, 0],
//                 |a, (i, (c, next_c))|
//                 [ if *c != '0' {a[0] + a[1]} else {0},
//                     if i>0 && (*c == '1' ||
//                     (*c == '2' && *next_c < '7')) {a[0]} else {0}
//                 ]
//             ).iter().sum()
//     }
// }

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let splice = s.as_bytes();
        if splice[0] == b'0' {
            return 0;
        }
        if s.len() == 1 {
            return 1;
        }
        let mut dp = vec![0; s.len() + 1];
        dp[0] = 1;
        dp[1] = if splice[0] == b'0' { 0 } else { 1 };
        splice.windows(2).enumerate().for_each(|(i, window)| {
            let one = window[1] - b'0';
            let two = (window[0] - b'0') * 10 + one;
            if (1..=9).contains(&one) {
                dp[i + 2] += dp[i + 1]
            }
            if (10..=26).contains(&two) {
                dp[i + 2] += dp[i]
            }
        });
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
