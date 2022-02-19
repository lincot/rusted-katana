#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const PHRASE: &str = "рандомизатор";

#[bench]
fn bench(bencher: &mut Bencher) {
    let phrase = black_box(PHRASE);

    bencher.iter(|| solution::solution(phrase))
}
