struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut count = [0; 26];
        words.iter().flat_map(|word| word.chars()).for_each(|c| {
            count[(c as u8 - b'a') as usize] += 1;
        });
        count.iter().all(|&c| c % words.len() as i32 == 0)
    }
}

fn main() {
    let input0 = vec![String::from("abc"), String::from("aabc"), String::from("bc")];
    let input1 = vec![String::from("ab"), String::from("a")];
    let output0 = Solution::make_equal(input0);
    let output1 = Solution::make_equal(input1);
    println!("{:?}", output0);
    println!("{:?}", output1);
}