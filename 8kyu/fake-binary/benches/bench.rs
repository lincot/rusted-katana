#![feature(test)]

extern crate test;
use fake_binary::fake_bin;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| fake_bin(black_box("366058562030849490134388085")));
}
