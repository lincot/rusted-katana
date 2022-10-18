#![no_std]
#![feature(test)]

extern crate test;
use dna_to_rna_conversion::dna_to_rna;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        dna_to_rna(black_box(
            "GCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCAT",
        ))
    });
}
