#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use core::array;
use operations_with_sequence::calc;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let array: [_; 1000] = array::from_fn(|_| rng.gen_range(0..1000));
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(calc(black_box(array.into())));
        }
    });
}
