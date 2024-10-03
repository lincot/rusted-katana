#![feature(test)]

extern crate test;
use credit_card_mask::maskify;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| maskify(black_box("4556364607935616")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| maskify(black_box("нет войне")));
}
