#![feature(test)]

extern crate test;
use beginner_reduce_but_grow::grow;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..100 {
            black_box(grow(black_box(vec![3; 8])));
        }
    });
}
