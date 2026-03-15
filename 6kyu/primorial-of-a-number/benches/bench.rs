#![feature(test)]

extern crate test;
use primorial_of_a_number::num_primorial;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| num_primorial(black_box(9)));
}
