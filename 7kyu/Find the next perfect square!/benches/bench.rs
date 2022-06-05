#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let sq = black_box(319_225);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::find_next_square(sq));
        }
    });
}
