#![feature(test)]

extern crate test;
use core::array;
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;
use smallest_unused_id::next_id;
use test::{black_box, Bencher};

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
            rng.gen()
        }
    });
    ids.shuffle(&mut rng);
    bencher.iter(|| next_id(black_box(&ids)));
}
