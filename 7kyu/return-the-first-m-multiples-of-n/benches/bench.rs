#![feature(test)]

extern crate test;
use return_the_first_m_multiples_of_n::multiples;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(multiples(black_box(100), black_box(3.15)));
        black_box(multiples(black_box(200), black_box(0.1)));
    });
}
