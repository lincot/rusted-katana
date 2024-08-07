#![feature(test)]

extern crate test;
use core::array;
use irreducible_sum_of_rationals::sum_fracts;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let l: [_; 100] = array::from_fn(|_| (rng.gen_range(1..=10), rng.gen_range(1..=10)));
    bencher.iter(|| sum_fracts(black_box(l.into())));
}
