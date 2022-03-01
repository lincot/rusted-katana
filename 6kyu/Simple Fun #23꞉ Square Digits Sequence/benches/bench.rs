#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let to = black_box(650);
    bencher.iter(|| {
        for a0 in 1..=to {
            black_box(solution::square_digits_sequence(a0));
        }
    })
}
