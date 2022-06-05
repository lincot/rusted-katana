#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("hello example (words(more words) here) something");
    bencher.iter(|| solution::remove_parentheses(s));
}
