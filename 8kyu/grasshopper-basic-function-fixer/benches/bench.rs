#![feature(test)]

extern crate test;
use grasshopper_basic_function_fixer::add_five;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| add_five(black_box(1000)));
}
