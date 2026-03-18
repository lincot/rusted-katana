#![feature(test)]

extern crate test;
use guess_the_sequence::sequence;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for x in 1..=if cfg!(miri) { 10 } else { 100 } {
            black_box(sequence(black_box(x)));
        }
    });
}
