//! <https://www.codewars.com/kata/62a933d6d6deb7001093de16/train/rust>

pub fn get_the_vowels(word: &str) -> u32 {
    let mut res = 0;
    let mut vowel_i = 0;
    let mut needed_vowel = b'a';
    for &b in word.as_bytes() {
        if b == needed_vowel {
            res += 1;
            if vowel_i == b"aeiou".len() - 1 {
                vowel_i = 0;
            } else {
                vowel_i += 1;
            }
            needed_vowel = *unsafe { b"aeiou".get_unchecked(vowel_i) };
        }
    }
    res
}
