#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use flatten_and_sort_an_array::flatten_and_sort;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let arr: [_; 16] = array::from_fn(|_| array::from_fn::<_, 1024, _>(|_| rng.gen()).into());
    bencher.iter(|| flatten_and_sort(black_box(&arr)));
}
