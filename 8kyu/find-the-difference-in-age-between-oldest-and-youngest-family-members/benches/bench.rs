#![feature(test)]

extern crate test;
use core::array;
use find_the_difference_in_age_between_oldest_and_youngest_family_members::difference_in_ages;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let lst: [_; 1024] = array::from_fn(|_| rng.gen());
    bencher.iter(|| difference_in_ages(black_box(&lst)));
}
