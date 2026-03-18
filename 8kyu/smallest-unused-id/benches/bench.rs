#![feature(test)]

extern crate test;
use core::array;
use rand::{RngExt, seq::SliceRandom};
use rand_pcg::Pcg64Mcg;
use smallest_unused_id::next_id;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const LEN: usize = if cfg!(miri) { 20 } else { 20_000 };
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut ids: [_; LEN] = array::from_fn(|i| {
        if i < LEN / 2 {
            i
        } else if i < LEN / 2 + LEN / 4 {
            2 * (i - LEN / 2)
        } else {
            rng.random::<u64>() as usize
        }
    });
    ids.shuffle(&mut rng);
    bencher.iter(|| next_id(black_box(&ids)));
}
