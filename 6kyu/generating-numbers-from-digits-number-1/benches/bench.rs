#![feature(test)]

extern crate test;
use core::array;
use generating_numbers_from_digits_number_1::proc_arr;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 100] = array::from_fn(|_| rng.gen_range(b'0'..=b'9') as char);
    bencher.iter(|| proc_arr(black_box(&arr)));
}
