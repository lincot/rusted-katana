#![feature(test)]

extern crate test;
use see_you_next_happy_year::next_happy_year;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for i in 1000..=9000 {
            black_box(next_happy_year(i));
        }
    });
}
