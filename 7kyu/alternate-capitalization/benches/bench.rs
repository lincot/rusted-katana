#![feature(test)]

extern crate test;
use alternate_capitalization::capitalize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("абракадабра");
    bencher.iter(|| capitalize(s));
}
