#![feature(test)]

extern crate test;
use remove_the_parentheses::remove_parentheses;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("hello example (words(more words) here) something");
    bencher.iter(|| remove_parentheses(s));
}
