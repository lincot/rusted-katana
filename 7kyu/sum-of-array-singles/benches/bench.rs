#![feature(test)]

extern crate test;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use sum_of_array_singles::repeats;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    // most upvoted solution needs Vec
    let mut arr: Vec<_> = (0..if cfg!(miri) { 64 } else { 1024 })
        .map(|i| {
            if i < if cfg!(miri) { 62 } else { 1022 } {
                i as _
            } else {
                1337 * i
            }
        })
        .collect();
    arr.shuffle(&mut rng);
    bencher.iter(|| repeats(black_box(&arr)));
}
