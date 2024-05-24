#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use simple_array_product::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let vecs: [_; 5] = array::from_fn(|_| {
        (0..rng.gen_range(0..1024))
            .map(|_| rng.gen_range(-20..20))
            .collect()
    });
    bencher.iter(|| solve(black_box(&vecs)));
}
