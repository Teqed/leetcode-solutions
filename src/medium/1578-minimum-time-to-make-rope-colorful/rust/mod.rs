struct Solution;

impl Solution {
    pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
        let mut output = 0;
        let mut needed_time = needed_time;
        let mut i = 0;
        colors.chars().collect::<Vec<char>>().windows(2).for_each(|x| {
            if x[0] == x[1] {
                if needed_time[i] < needed_time[i+1] {
                    output += needed_time[i];
                } else {
                    output += needed_time[i+1];
                    needed_time[i+1] = needed_time[i];
                }
            }
            i += 1;
        });
        output
    }
}

fn main() {
    let input = String::from("ababa");
    let input2 = vec![10,5,10,3,2];
    println!("Input: colors = {:?}, neededTime = {:?}", input, input2);
    println!("Output: {:?}", Solution::min_cost(input, input2));
    let input = String::from("aaabbbabbbb");
    let input2 = vec![3,5,10,7,5,3,5,5,4,8];
    println!("Input: colors = {:?}, neededTime = {:?}", input, input2);
    println!("Output: {:?}", Solution::min_cost(input, input2));
}