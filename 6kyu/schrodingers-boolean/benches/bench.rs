#![feature(test)]

extern crate test;
use schrodingers_boolean::omnibool;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(black_box(omnibool) == true);
        black_box(black_box(omnibool) == false);
    });
}
