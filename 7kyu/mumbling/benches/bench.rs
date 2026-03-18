#![feature(test)]

extern crate test;
use mumbling::accum;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| accum(black_box("ZpglnRxqenU")));
}
