#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use split_and_then_add_both_sides_of_an_array_together::split_and_add;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 100] = array::from_fn(|_| rng.gen_range(0..u32::MAX >> 4));
    bencher.iter(|| split_and_add(black_box(&arr), black_box(4)));
}
