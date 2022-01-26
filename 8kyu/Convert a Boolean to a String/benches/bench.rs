#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const B: bool = true;

#[bench]
fn bench(bencher: &mut Bencher) {
    let b = black_box(B);

    bencher.iter(|| {
        for _ in 0..1_000_000 {
            black_box(solution::boolean_to_string(b));
        }
    })
}
