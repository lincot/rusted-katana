#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: u64 = 5123456789;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(N);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::tidy_number(n));
        }
    })
}
