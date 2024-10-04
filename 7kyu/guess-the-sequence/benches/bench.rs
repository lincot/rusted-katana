#![feature(test)]

extern crate test;
use guess_the_sequence::sequence;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for x in 1..=if cfg!(miri) { 10 } else { 100 } {
            black_box(sequence(black_box(x)));
        }
    });
}
