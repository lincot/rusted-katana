#![feature(test)]

extern crate test;
use core::array;
use odd_heavy_array::is_odd_heavy;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench_random(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| rng.gen());
    bencher.iter(|| is_odd_heavy(black_box(&arr)));
}

#[bench]
fn bench_odd_heavy(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|_| {
        if rng.gen() {
            let x: i32 = rng.gen();
            x.abs() - x.abs() % 2 + 1
        } else {
            let x: i32 = rng.gen();
            -(x.abs() - x.abs() % 2)
        }
    });
    bencher.iter(|| is_odd_heavy(black_box(&arr)));
}
