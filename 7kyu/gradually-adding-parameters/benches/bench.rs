#![feature(test)]

extern crate test;
use core::array;
use gradually_adding_parameters::add;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let args: [_; 1024] = array::from_fn(|_| rng.gen_range(0..100));
    bencher.iter(|| add(black_box(&args)));
}
