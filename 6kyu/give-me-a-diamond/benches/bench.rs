#![feature(test)]

extern crate test;
use give_me_a_diamond::print;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| print(black_box(101)));
}
