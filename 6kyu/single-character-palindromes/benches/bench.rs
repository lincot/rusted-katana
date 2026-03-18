#![feature(test)]

extern crate test;
use single_character_palindromes::solve;
use test::{Bencher, black_box};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box("drawnonwaird")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box("дравнонваирд")));
}
