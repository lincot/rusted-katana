#![feature(test)]

extern crate test;
use counting_valleys::counting_valleys;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        counting_valleys(black_box(
            "UFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFUUFFDDFDUDFUFU",
        ))
    });
}
