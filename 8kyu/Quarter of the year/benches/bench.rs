#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const MONTH: u8 = 3;

#[bench]
fn bench(bencher: &mut Bencher) {
    let month = black_box(MONTH);

    bencher.iter(|| {
        for _ in 0..1_000_000 {
            black_box(solution::quarter_of(month));
        }
    })
}
