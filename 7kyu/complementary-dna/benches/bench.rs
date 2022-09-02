#![feature(test)]

extern crate test;
use complementary_dna::dna_strand;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let dna = black_box("GCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCATGCAT");
    bencher.iter(|| dna_strand(dna));
}
