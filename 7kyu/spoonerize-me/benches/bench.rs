#![feature(test)]

extern crate test;
use spoonerize_me::spoonerize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let words = black_box("Дионисий Младший");
    bencher.iter(|| spoonerize(words));
}
