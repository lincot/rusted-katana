#![feature(test)]

extern crate test;
use spoonerize_me::spoonerize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| spoonerize(black_box("Дионисий Младший")));
}
