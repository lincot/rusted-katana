#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use simple_fun_number_170_sum_groups::sum_groups;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 1000] = array::from_fn(|_| rng.gen_range(0..1000));
    bencher.iter(|| sum_groups(black_box(&arr)));
}
