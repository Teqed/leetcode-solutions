use std::collections::HashMap;

const ORDINAL_A: u8 = b'a'; // Ordinal value of 'a' in ASCII table
const ALPHABET_LENGTH: usize = (b'z' - ORDINAL_A + 1) as _; // Number of letters in the alphabet

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter().fold(HashMap::<[u8; ALPHABET_LENGTH], Vec<String>>::new(), |mut grouped_anagrams, word| {
            grouped_anagrams.entry(word.bytes().map(|ordinal_character| (ordinal_character - ORDINAL_A) as usize).fold([0; ALPHABET_LENGTH], |mut frequency, character| {
                frequency[character] += 1;
                frequency
            })).or_default().push(word);
            grouped_anagrams
        }).into_values().collect()
    }
}

fn main() {
    let input = vec!["eat", "tea", "tan", "ate", "nat", "bat"].into_iter().map(|s| s.to_string()).collect();
    println!("Input: {:?}", input);
    println!("Output: {:?}", Solution::group_anagrams(input));
}
