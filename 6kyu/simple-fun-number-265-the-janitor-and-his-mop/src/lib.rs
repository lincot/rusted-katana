//! <https://www.codewars.com/kata/59128363e5bc24091a00006f/train/rust>

pub fn the_janitor(word: &str) -> [usize; 26] {
    let mut first = [0; 26];
    let mut last = [0; 26];
    for (i, &b) in word.as_bytes().iter().enumerate() {
        let letter_i = b - b'a';
        if letter_i < 26 {
            if first[letter_i as usize] == 0 {
                first[letter_i as usize] = i + 1;
            }
            last[letter_i as usize] = i + 2;
        }
    }
    for i in 0..26 {
        last[i] -= first[i];
    }
    last
}
