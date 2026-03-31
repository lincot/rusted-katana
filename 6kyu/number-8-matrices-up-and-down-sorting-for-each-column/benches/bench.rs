#![feature(test)]

extern crate test;
use core::array;

use number_8_matrices_up_and_down_sorting_for_each_column::up_down_col_sort;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const N: usize = if cfg!(miri) { 10 } else { 100 };
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let matrix: [_; N] = array::from_fn(|_| (0..N).map(|_| rng.random()).collect());
    bencher.iter(|| up_down_col_sort(black_box(&matrix)));
}
