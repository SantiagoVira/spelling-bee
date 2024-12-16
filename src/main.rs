// Word list from: https://github.com/dwyl/english-words/blob/master/words.txt
// Other word list at https://github.com/raun/Scrabble/blob/master/words.txt (doesn't contain words with more than 8 letters)

use std::fs;

fn is_pangram(word: &str, letters: &str) -> bool {
    let center_letter = letters.chars().nth(3).unwrap();
    // Check for length
    if word.len() < 7 {
        return false;
    }
    if !word.contains(center_letter) {
        return false;
    }
    if !letters.chars().all(|c| word.contains(c)) {
        return false;
    }
    if !word.chars().all(|c| letters.contains(c)) {
        return false;
    }

    return true;
}

fn main() {
    let letters = "IZNEHWG";
    let wordfile_contents =
        fs::read_to_string("src/words.txt").expect("Should have been able to read the file");

    let pangrams = wordfile_contents
        .split("\n")
        .filter(|word| is_pangram(&word.to_uppercase(), letters))
        .collect::<Vec<_>>();

    println!("{:?}", pangrams)
}
