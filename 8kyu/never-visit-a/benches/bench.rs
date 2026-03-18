#![feature(test)]

extern crate test;
use never_visit_a::subtract_sum;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| subtract_sum(black_box(1000)));
}
