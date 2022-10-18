#![no_std]
#![feature(test)]

extern crate test;
use consecutive_vowels_in_a_string::get_the_vowels;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_the_vowels(black_box("akfheujfkgiaaaofmmfkdfuaiiie")));
}
