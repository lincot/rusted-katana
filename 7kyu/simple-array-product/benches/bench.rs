#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec::Vec;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use simple_array_product::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let vecs: [_; 100] = array::from_fn(|_| {
        let len = rng.gen_range(0..1024);
        let mut res = Vec::with_capacity(len);
        unsafe { res.set_len(len) };
        let mut ptr = res.as_mut_ptr();
        for _ in 0..len {
            unsafe {
                *ptr = rng.gen_range(-20..20);
                ptr = ptr.add(1);
            }
        }
        res
    });
    bencher.iter(|| solve(black_box(&vecs)));
}
