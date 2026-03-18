#![feature(test)]

extern crate test;
use create_new_programming_mechanism_to_add_numbers::add;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| add![black_box(2), black_box(3), black_box(4)]);
}
