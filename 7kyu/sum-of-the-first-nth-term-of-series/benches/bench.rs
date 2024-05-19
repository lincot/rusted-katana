#![feature(test)]

extern crate test;
use sum_of_the_first_nth_term_of_series::series_sum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| series_sum(black_box(39)));
}
