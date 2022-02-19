#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const S: &str =
    "When you in order to do something by a wrong way, your heart will missed somewhere.";

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(S);

    bencher.iter(|| solution::testit(s))
}
