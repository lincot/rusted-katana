#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use sorting_on_planet_twisted_3_7::sort_twisted_37;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let list: [_; 1000] = array::from_fn(|_| rng.gen_range(0..100));
    bencher.iter(|| sort_twisted_37(black_box(&list)));
}
