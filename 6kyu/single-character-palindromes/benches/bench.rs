#![feature(test)]

extern crate test;
use single_character_palindromes::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("дравнонваирд");
    bencher.iter(|| solve(s));
}
