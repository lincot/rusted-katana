//! <https://www.codewars.com/kata/62a933d6d6deb7001093de16/train/rust>

#![no_std]

pub fn get_the_vowels(word: &str) -> u32 {
    let mut res = 0;
    let mut vowels_cycle = b"aeiou".iter().cycle();
    let mut needed_vowel = vowels_cycle.next().unwrap();
    for b in word.as_bytes() {
        if b == needed_vowel {
            res += 1;
            needed_vowel = unsafe { vowels_cycle.next().unwrap_unchecked() };
        }
    }
    res
}
