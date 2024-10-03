#![feature(test)]

extern crate test;
use spoonerize_me::spoonerize;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| spoonerize(black_box("Dionysius Younger")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| spoonerize(black_box("Дионисий Младший")));
}
