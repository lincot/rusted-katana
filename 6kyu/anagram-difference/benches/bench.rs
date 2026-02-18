#![feature(test)]

extern crate test;
use anagram_difference::anagram_difference;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| anagram_difference(black_box("codewars"), black_box("hackerrank")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| anagram_difference(black_box("цодеварс"), black_box("хацкерранк")));
}
