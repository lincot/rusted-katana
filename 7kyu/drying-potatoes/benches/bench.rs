#![feature(test)]

extern crate test;
use drying_potatoes::potatoes;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..5 {
            black_box(potatoes(black_box(99), black_box(100), black_box(98)));
        }
    });
}
