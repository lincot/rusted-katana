#![no_std]
#![feature(test)]

extern crate test;
use adjacent_repeated_words_in_a_string::count_adjacent_pairs;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_adjacent_pairs(black_box("dog cat dog DOG cat apple dog cat pineapple apple dog cat apple apple dog cat apple dog apple dog cat dog dog DOG dog dog dog dog dog dog dog cat cat cat cat dog dog cat cat")));
}
