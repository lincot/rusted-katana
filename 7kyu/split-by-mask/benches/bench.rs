#![feature(test)]

extern crate test;
use core::array;

use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use split_by_mask::split;
use test::{Bencher, black_box};
use unchecked_std::PushUnchecked;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let xs: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|i| b'a' + (i % 26) as u8);
    let xs = unsafe { core::str::from_utf8_unchecked(&xs) };
    let mut mask_sum = 0;
    let mut mask = Vec::with_capacity(xs.len());
    loop {
        let len = rng.random_range(0..50).min(xs.len() - mask_sum);
        unsafe { mask.push_unchecked(len) };
        mask_sum += len;
        if mask_sum == xs.len() {
            break;
        }
    }
    bencher.iter(|| split(black_box(xs), black_box(&mask)));
}
