#![feature(test)]

extern crate test;
use core::array;
use n_smallest_elements_in_original_order::first_n_smallest;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let arr: [_; 1000] = array::from_fn(|_| rng.gen());
    bencher.iter(|| first_n_smallest(black_box(&arr), black_box(30)));
}
