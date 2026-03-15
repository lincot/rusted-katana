#![feature(test)]

extern crate test;
use powers_of_i::pofi;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pofi(black_box(10)));
}
