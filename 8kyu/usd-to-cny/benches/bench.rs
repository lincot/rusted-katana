#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use usd_to_cny::usdcny;

#[bench]
fn bench(bencher: &mut Bencher) {
    let usd = black_box(465);
    bencher.iter(|| usdcny(usd));
}
