#![feature(test)]

extern crate test;
use dont_give_me_five::dont_give_me_five;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let start = black_box(456_439);
    let end = black_box(1_837_815);
    bencher.iter(|| dont_give_me_five(start, end));
}
