#![no_std]
#![feature(test)]

extern crate test;
use complementary_dna::dna_strand;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        dna_strand(black_box(
            "GCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCAT",
        ))
    });
}
