#![feature(test)]

extern crate test;
use esthetic_numbers::esthetic;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for num in [
            10, 23, 666, 13, 1, 9, 74, 740, 928, 259_259, 883_271, 1_080_898, 1_080_899,
        ] {
            black_box(esthetic(black_box(num)));
        }
    });
}
