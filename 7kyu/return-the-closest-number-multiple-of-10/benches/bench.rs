#![feature(test)]

extern crate test;
use return_the_closest_number_multiple_of_10::closest_multiple_of_10;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(closest_multiple_of_10(black_box(866_703)));
        }
    });
}
