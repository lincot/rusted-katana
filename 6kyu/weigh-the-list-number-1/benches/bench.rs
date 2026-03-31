#![feature(test)]

extern crate test;
use core::array;

use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};
use weigh_the_list_number_1::weigh_the_list;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let a: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| rng.random());
    bencher.iter(|| weigh_the_list(black_box(a.into())));
}
