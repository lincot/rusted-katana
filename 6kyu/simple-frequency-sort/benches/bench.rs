#![feature(test)]

extern crate test;
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;
use simple_frequency_sort::solve;
use test::{black_box, Bencher};
use unchecked_std::prelude::*;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let len = if cfg!(miri) { 50 } else { 500 };
    let mut vec = Vec::with_capacity(3 * len);
    for _ in 0..len {
        unsafe { vec.push_unchecked(rng.gen()) };
    }
    for i in 0..len {
        if rng.gen_ratio(1, 4) {
            unsafe { vec.push_unchecked(vec[i]) };
            if rng.gen() {
                unsafe { vec.push_unchecked(vec[i]) };
            }
        }
    }
    vec.shuffle(&mut rng);
    bencher.iter(|| solve(black_box(&vec)));
}
