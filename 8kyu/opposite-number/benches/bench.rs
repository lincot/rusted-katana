#![feature(test)]

extern crate test;
use opposite_number::opposite;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..10 {
            black_box(opposite(black_box(-1000)));
        }
    });
}
