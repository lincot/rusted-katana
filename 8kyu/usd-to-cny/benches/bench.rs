#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use usd_to_cny::usdcny;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| usdcny(black_box(465)));
}
