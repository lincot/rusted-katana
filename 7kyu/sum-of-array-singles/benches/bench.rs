#![feature(test)]

extern crate test;
use core::array;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use sum_of_array_singles::repeats;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let mut arr: [_; 1024] = array::from_fn(|i| if i < 1022 { i as _ } else { 1337 * i as i32 });
    arr.shuffle(&mut rng);
    bencher.iter(|| repeats(black_box(&arr)));
}
