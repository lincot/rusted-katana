#![feature(test)]

extern crate test;
use core::array;

use difference_of_2::twos_difference;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const LEN: usize = if cfg!(miri) { 16 } else { 1024 };
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut used = [false; 4 * LEN];
    let arr: [_; LEN] = array::from_fn(|_| {
        loop {
            let val = rng.random_range(0..4 * LEN as u32);
            if !used[val as usize] {
                used[val as usize] = true;
                break val;
            }
        }
    });
    bencher.iter(|| twos_difference(black_box(&arr)));
}
