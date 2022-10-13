#![no_std]
#![feature(test)]

extern crate test;
use counting_valleys::counting_valleys;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU");
    bencher.iter(|| counting_valleys(s));
}
