#![feature(test)]

extern crate test;
use core::array;
use merge_two_sorted_arrays_into_one::merge_arrays;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let [mut arr1, mut arr2]: [[_; 100]; 2] =
        array::from_fn(|_| array::from_fn(|_| rng.gen_range(-100..100)));
    vqsort_rs::sort(&mut arr1);
    vqsort_rs::sort_descending(&mut arr2);
    bencher.iter(|| merge_arrays(black_box(&arr1), black_box(&arr2)));
}
