struct Solution;

impl Solution {
    pub fn max_width_of_vertical_area(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_unstable_by_key(|point| point[0]);
        return points.windows(2).map(|point| point[1][0] - point[0][0]).max().unwrap();
    }
}

fn main() {
    let input = vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]];
    println!("Input: {:?}", input);
    println!("Output: {:?}", Solution::max_width_of_vertical_area(input));
}
