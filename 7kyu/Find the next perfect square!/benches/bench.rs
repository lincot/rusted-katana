#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const SQ: u64 = 319_225;

#[bench]
fn bench(bencher: &mut Bencher) {
    let sq = black_box(SQ);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::find_next_square(sq));
        }
    })
}
