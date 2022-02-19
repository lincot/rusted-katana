#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const WORD: &str = "király";

#[bench]
fn bench(bencher: &mut Bencher) {
    let word = black_box(WORD);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::dative(word));
        }
    })
}
