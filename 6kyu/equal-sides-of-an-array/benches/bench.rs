#![feature(test)]

extern crate test;
use core::array;
use equal_sides_of_an_array::find_even_index;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 64 } else { 1024 }] =
        array::from_fn(|_| rng.random_range(-1000..1000));
    bencher.iter(|| find_even_index(black_box(&arr)));
}
