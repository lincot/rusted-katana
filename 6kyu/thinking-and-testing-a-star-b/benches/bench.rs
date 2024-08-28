#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use thinking_and_testing_a_star_b::test_it;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| test_it(black_box(123_456_789), black_box(987_654_321_012_345)));
}
