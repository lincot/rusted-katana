#![feature(test)]

extern crate test;
use multiples_of_3_or_5::solution;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box(45643)));
}
