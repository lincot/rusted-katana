#![feature(test)]

extern crate test;
use hex_to_decimal::hex_to_dec;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(hex_to_dec(black_box("abc145fb")));
        black_box(hex_to_dec(black_box("123123ab")));
    });
}
