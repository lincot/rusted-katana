#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let start = black_box(456_439);
    let end = black_box(1_837_815);
    bencher.iter(|| solution::dont_give_me_five(start, end));
}
