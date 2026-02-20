#![feature(test)]

extern crate test;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};
use thinkful_list_and_loop_drills_inverse_slicer::inverse_slice;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let input: [usize; 1024] = array::from_fn(|_| rng.random::<u64>() as usize);
    bencher.iter(|| inverse_slice(black_box(&input), black_box(50), black_box(125)));
}
