#![no_std]
#![feature(test)]

extern crate test;
use dna_to_rna_conversion::dna_to_rna;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let dna = black_box("GCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCAT");
    bencher.iter(|| dna_to_rna(dna));
}
