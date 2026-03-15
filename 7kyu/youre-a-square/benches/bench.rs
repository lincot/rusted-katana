#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use youre_a_square::is_square;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(is_square(black_box(25)));
        black_box(is_square(black_box(26)));
    });
}
