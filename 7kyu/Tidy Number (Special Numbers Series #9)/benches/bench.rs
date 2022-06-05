#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(5_123_456_789);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::tidy_number(n));
        }
    });
}
