#![feature(test)]

extern crate test;
use dont_give_me_five::dont_give_me_five;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| dont_give_me_five(black_box(456_439), black_box(1_837_815)));
}
