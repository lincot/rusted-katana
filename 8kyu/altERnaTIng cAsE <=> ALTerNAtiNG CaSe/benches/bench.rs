#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str = "пРиВеТ МииР";

#[bench]
fn bench(b: &mut Bencher) {
    let s = black_box(S);

    b.iter(|| solution::to_alternating_case(s))
}
