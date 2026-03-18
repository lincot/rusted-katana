#![feature(test)]

extern crate test;
use opposites_attract::lovefunc;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| lovefunc(black_box(15), black_box(20)));
}
