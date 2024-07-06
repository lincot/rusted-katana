#![feature(test)]

extern crate test;
use pandemia::infected;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| infected(black_box("01000000X000X011X0X")));
}
