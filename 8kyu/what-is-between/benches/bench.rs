#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use what_is_between::between;

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(-30000);
    let b = black_box(30000);
    bencher.iter(|| between(a, b));
}
