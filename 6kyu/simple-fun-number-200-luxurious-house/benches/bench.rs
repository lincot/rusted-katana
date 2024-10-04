#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use simple_fun_number_200_luxurious_house::luxhouse;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let houses: [_; if cfg!(miri) { 64 } else { 1000 }] = array::from_fn(|_| rng.gen_range(0..300));
    bencher.iter(|| luxhouse(black_box(&houses)));
}
