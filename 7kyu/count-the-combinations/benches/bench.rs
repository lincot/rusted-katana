#![feature(test)]

extern crate test;
use count_the_combinations::num_combo;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let array = [1; 1000];
    bencher.iter(|| num_combo(black_box(&array), black_box(array.len() as u32 - 1)));
}
