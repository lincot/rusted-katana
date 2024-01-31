#![feature(test)]

extern crate test;
use core::array;
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64;
use smallest_unused_id::next_id;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const LEN: usize = if cfg!(miri) { 20 } else { 20_000 };
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
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
