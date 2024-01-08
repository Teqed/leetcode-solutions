#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    clippy::suspicious,
    clippy::unwrap_used,
    clippy::question_mark_used
)]

struct Solution;

impl Solution {
    const DELIMITER: char = '\0';

    pub fn encode(strs: Vec<&str>) -> String {
        let mut result = String::new();
        for s in strs {
            result.push_str(s);
            result.push(Self::DELIMITER);
        }
        result
    }

    pub fn decode(s: &str) -> Vec<String> {
        let mut result = Vec::new();
        let mut start = 0;
        for (end, ch) in s.char_indices() {
            if ch == Self::DELIMITER {
                result.push(s[start..end].to_string());
                start = end + ch.len_utf8();
            }
        }
        result
    }
}

fn main() {
    let test_cases = vec![
        vec!["basic","string","of","characters"],
        vec!["dog","cat",":","true"],
        vec![""],
        vec![],
        vec!["fish","fish",":","false","!@#$%^&*()"],
        vec!["#","##"],
        vec!["1,23","45,6","7,8,9"],
        vec!["hello","world","hello"],
        vec!["a","b","c","d"],
        vec!["@#$","%^&*","!@#$%^&*"],
        vec!["apple","orange","banana","grape","kiwi","melon"],
        vec!["The quick brown fox","jumps over the","lazy dog","1234567890","abcdefghijklmnopqrstuvwxyz"],
        vec!["","   ","!@#$%^&*()_+","LongStringWithNoSpaces","Another, String With, Commas"],
        vec!["String with new\nline","Another\nLine","And\nOne\nMore"],
        vec!["123","456","789","10","11","12","13","14","15","16","17","18","19","20"],
        vec!["Repeated","Repeated","Repeated","Repeated","Repeated","Repeated"],
        vec!["SingleCharacter","A","B","C","D","E","F","G","H","I","J"],
        vec!["EmojiTest 😊","🌟✨🌟✨🌟","🤖👽🤖👽"],
        vec!["Strings with spaces are tricky","Another one with spaces","This also has spaces"],
        vec!["VeryLongStringWithoutAnySpacesOrSpecialCharactersRepeatedManyTimesVeryLongStringWithoutAnySpacesOrSpecialCharactersRepeatedManyTimes"]
    ];

    for test_case in test_cases {
        let encoded = Solution::encode(test_case.clone());
        let decoded = Solution::decode(&encoded);
        println!("Input: {test_case:?}");
        println!("Encoded: {encoded:?}");
        println!("Decoded: {decoded:?}");
        assert_eq!(test_case, decoded);
        println!("Success!\n");
    }
}
