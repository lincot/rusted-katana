#![feature(test)]

extern crate test;
use core::array;
use enumerable_magic_number_20_cascading_subsets::each_cons;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 100] = array::from_fn(|_| rng.gen_range(0..=20));
    bencher.iter(|| each_cons(black_box(&arr), black_box(50)));
}
