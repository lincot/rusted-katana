#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const PHRASE: &str = "рандомизатор";

#[bench]
fn bench(b: &mut Bencher) {
    let phrase = black_box(PHRASE);

    b.iter(|| solution::solution(phrase))
}
