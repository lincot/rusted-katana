#![feature(test)]

extern crate test;
use credit_card_mask::maskify;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| maskify(black_box("нет войне")));
}
