#![feature(test)]

extern crate test;
use core::array;
use divide_and_conquer::div_con;
use either::Either;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| {
        if rng.random() {
            Either::Left(rng.random_range(0..10))
        } else {
            Either::Right(unsafe {
                String::from_utf8_unchecked(vec![b'0' + rng.random_range(0..10)])
            })
        }
    });
    bencher.iter(|| div_con(black_box(&arr)));
}
