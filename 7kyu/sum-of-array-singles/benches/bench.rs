#![feature(test)]

extern crate test;
use core::array;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use sum_of_array_singles::repeats;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut arr: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|i| {
        if i < if cfg!(miri) { 62 } else { 1022 } {
            i as _
        } else {
            1337 * i as i32
        }
    });
    arr.shuffle(&mut rng);
    bencher.iter(|| repeats(black_box(&arr)));
}
