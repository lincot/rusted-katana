#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const VERB: &str = "vivir";

#[bench]
fn bench(bencher: &mut Bencher) {
    let verb = black_box(VERB);

    bencher.iter(|| solution::conjugate(verb))
}
