#![feature(test)]

extern crate test;
use happy_birthday_1::wrap;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| wrap(black_box(17), black_box(32), black_box(11)));
}
