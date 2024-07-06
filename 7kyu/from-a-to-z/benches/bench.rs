#![feature(test)]

extern crate test;
use from_a_to_z::gimme_the_letters;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(gimme_the_letters(black_box("e-k")));
        black_box(gimme_the_letters(black_box("B-Y")));
    });
}
