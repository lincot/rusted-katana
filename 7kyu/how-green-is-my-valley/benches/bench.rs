#![feature(test)]

extern crate test;
use core::array;
use how_green_is_my_valley::make_valley;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 300] = array::from_fn(|_| rng.gen());
    bencher.iter(|| make_valley(black_box(arr.into())));
}
