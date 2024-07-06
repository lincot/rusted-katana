#![feature(test)]

extern crate test;
use hamming_distance::hamming;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        hamming(
            black_box("old father, old artificer"),
            black_box("of my soul the uncreated "),
        )
    });
}
