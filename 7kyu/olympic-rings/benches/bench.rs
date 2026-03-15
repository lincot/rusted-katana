#![feature(test)]

extern crate test;
use olympic_rings::olympic_ring;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| olympic_ring(black_box("eCEHWEPwwnvzMicyaRjk")));
}
