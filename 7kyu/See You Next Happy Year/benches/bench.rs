#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for i in 1000..=9000 {
            black_box(solution::next_happy_year(i));
        }
    });
}
