#![feature(test)]

extern crate test;
use fake_binary::fake_bin;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("366058562030849490134388085");
    bencher.iter(|| fake_bin(s));
}
