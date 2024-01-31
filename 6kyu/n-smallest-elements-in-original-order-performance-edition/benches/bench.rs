#![feature(test)]

extern crate test;
use core::array;
use n_smallest_elements_in_original_order_performance_edition::performant_smallest;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let arr: [_; 1024] = array::from_fn(|_| rng.gen_range(1..51));
    bencher.iter(|| performant_smallest(black_box(&arr), black_box(30)));
}
