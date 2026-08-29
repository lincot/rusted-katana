#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use two_sets_of_equal_sum::create_two_sets_of_equal_sum;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(create_two_sets_of_equal_sum(black_box(400)));
        black_box(create_two_sets_of_equal_sum(black_box(403)));
    });
}
