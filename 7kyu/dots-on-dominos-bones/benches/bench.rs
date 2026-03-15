#![feature(test)]

extern crate test;
use dots_on_dominos_bones::dots_on_domino_bones;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| dots_on_domino_bones(black_box(999)));
}
