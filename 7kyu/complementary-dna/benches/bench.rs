#![feature(test)]

extern crate test;
use complementary_dna::dna_strand;
use core::array;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let dna: [_; if cfg!(miri) { 64 } else { 8192 }] = array::from_fn(|i| b"GCAT"[i % 4]);
    bencher.iter(|| dna_strand(black_box(unsafe { core::str::from_utf8_unchecked(&dna) })));
}
