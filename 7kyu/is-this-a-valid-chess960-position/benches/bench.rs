#![feature(test)]

extern crate test;
use is_this_a_valid_chess960_position::is_valid;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(is_valid(black_box("RNBQKBNR")));
        }
    });
}
