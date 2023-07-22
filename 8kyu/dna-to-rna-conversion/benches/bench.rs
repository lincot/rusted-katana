#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use dna_to_rna_conversion::dna_to_rna;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let dna: [_; 8192] = array::from_fn(|i| b"GCAT"[i % 4]);
    bencher.iter(|| dna_to_rna(black_box(unsafe { core::str::from_utf8_unchecked(&dna) })));
}
