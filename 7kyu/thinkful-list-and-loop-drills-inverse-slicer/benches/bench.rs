#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};
use thinkful_list_and_loop_drills_inverse_slicer::inverse_slice;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let input: [usize; 1024] = array::from_fn(|_| rng.gen());
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(inverse_slice(
                black_box(&input),
                black_box(50),
                black_box(125),
            ));
        }
    });
}
