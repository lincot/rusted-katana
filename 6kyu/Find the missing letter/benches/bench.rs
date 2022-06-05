#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let chars = black_box(&['a', 'b', 'c', 'd', 'f']);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::find_missing_letter(chars));
        }
    });
}
