#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const NAME: &str = "Дмитрий Муратов";

#[bench]
fn bench(bencher: &mut Bencher) {
    let name = black_box(NAME);

    bencher.iter(|| solution::abbrev_name(name))
}
