#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("пРиВеТ МииР 2020");
    bencher.iter(|| solution::to_alternating_case(s))
}
