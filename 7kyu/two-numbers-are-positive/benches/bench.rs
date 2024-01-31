#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use two_numbers_are_positive::two_are_positive;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(two_are_positive(black_box(1), black_box(-1), black_box(1)));
            black_box(two_are_positive(black_box(1), black_box(1), black_box(1)));
            black_box(two_are_positive(
                black_box(-1),
                black_box(-1),
                black_box(-1),
            ));
            black_box(two_are_positive(black_box(1), black_box(-1), black_box(-1)));
        }
    });
}
