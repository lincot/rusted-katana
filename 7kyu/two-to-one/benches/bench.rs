#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use two_to_one::longest;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| longest(black_box("aretheyhere"), black_box("yestheyarehere")));
}
