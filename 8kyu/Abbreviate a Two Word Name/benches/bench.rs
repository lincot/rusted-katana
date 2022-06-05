#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let name = black_box("Дмитрий Муратов");
    bencher.iter(|| solution::abbrev_name(name));
}
