#![feature(test)]

extern crate test;
use hanoi_record::hanoi;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| hanoi(black_box(50)));
}
