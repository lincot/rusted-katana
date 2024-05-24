#![feature(test)]

extern crate test;
use core::array;
use fold_an_array::fold_array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 100] = array::from_fn(|_| rng.gen_range(-1000..1000));
    bencher.iter(|| fold_array(black_box(&arr), black_box(5)));
}
