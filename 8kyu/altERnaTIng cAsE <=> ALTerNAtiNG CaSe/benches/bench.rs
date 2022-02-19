#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "пРиВеТ МииР 2020";

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(S);

    bencher.iter(|| solution::to_alternating_case(s))
}
