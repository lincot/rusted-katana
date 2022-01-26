#![feature(test)]

extern crate test;
use solution;
use test::{black_box, Bencher};

const NAME: &str = "Дмитрий Муратов";

#[bench]
fn bench(b: &mut Bencher) {
    let name = black_box(NAME);

    b.iter(|| solution::abbrev_name(name))
}
