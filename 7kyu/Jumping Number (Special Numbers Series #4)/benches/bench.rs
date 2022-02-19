#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(98789876);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::jumping_number(n));
        }
    })
}
