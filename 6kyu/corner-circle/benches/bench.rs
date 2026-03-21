#![feature(test)]

extern crate test;
use corner_circle::corner_circle;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| corner_circle(black_box(17.)));
}
