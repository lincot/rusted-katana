#![feature(test)]

extern crate test;
use core::array;
use even_odd_disparity::solve;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let v: [_; 1024] = array::from_fn(|_| {
        let mut res = [b'0'; 10];
        *res.last_mut().unwrap() = rng.gen_range(b'0'..=b'9' + 1);
        unsafe { String::from_utf8_unchecked(res.into()) }
    });
    bencher.iter(|| solve(black_box(&v)));
}
