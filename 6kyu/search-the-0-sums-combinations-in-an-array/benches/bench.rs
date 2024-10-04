#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use search_the_0_sums_combinations_in_an_array::find_zero_sum_groups;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 20] = array::from_fn(|_| rng.gen_range(-10..=10));
    bencher.iter(|| find_zero_sum_groups(black_box(&arr), black_box(3)));
}
