#![feature(test)]

extern crate test;
use core::array;
use divide_and_conquer::div_con;
use either::Either;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let arr: [_; 1024] = array::from_fn(|_| {
        if rng.gen() {
            Either::Left(rng.gen_range(0..10))
        } else {
            Either::Right(unsafe { String::from_utf8_unchecked(vec![b'0' + rng.gen_range(0..10)]) })
        }
    });
    bencher.iter(|| div_con(black_box(&arr)));
}
