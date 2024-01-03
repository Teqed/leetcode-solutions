struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        fn count_laser_beams(bank: &[String], mut row: usize) -> i32 {
            let mut laser_beams = 0;
            while let Some(next_row) = bank.iter().skip(row + 1).position(|row| row.contains('1')) {
                laser_beams += bank[row].matches('1').count() as i32 * bank[next_row + row + 1].matches('1').count() as i32;
                row = next_row + row + 1;
            }
            laser_beams
        }
        bank.iter().position(|row| row.contains('1')).map_or(0, |row| count_laser_beams(&bank, row))
    }
}

fn main() {
    // ["011001","000000","010100","001000"]
    let input0 = vec!["011001".to_string(), "000000".to_string(), "010100".to_string(), "001000".to_string()];
    let output0 = Solution::number_of_beams(input0);
    println!("{output0}", output0=output0); // Should be 8
    // ["000","111","000"]
    let input1 = vec!["000".to_string(), "111".to_string(), "000".to_string()];
    let output1 = Solution::number_of_beams(input1);
    println!("{output1}", output1=output1); // Should be 0
}
