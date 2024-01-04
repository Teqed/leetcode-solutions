struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![1; nums.len()];
        let mut left = 1;
        let mut right = 1;
        for i in 0..nums.len() {
            output[i] *= left;
            output[nums.len() - 1 - i] *= right;
            left *= nums[i];
            right *= nums[nums.len() - 1 - i];
        }
        output
    }
}

fn main() {
    let input = vec![1, 2, 3, 4];
    println!("Input: {:?}", input);
    println!("Output: {:?}", Solution::product_except_self(input));
    let input2 = vec![-1, 1, 0, -3, 3];
    println!("Input: {:?}", input2);
    println!("Output: {:?}", Solution::product_except_self(input2));
}
