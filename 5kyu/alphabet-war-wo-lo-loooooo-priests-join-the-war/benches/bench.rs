#![feature(test)]

extern crate test;
use core::array;

use alphabet_war_wo_lo_loooooo_priests_join_the_war::alphabet_war;
use rand::seq::IndexedRandom;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let fight: [_; if cfg!(miri) { 16 } else { 1000 }] =
        array::from_fn(|_| *b"wpbsmqdztj".choose(&mut rng).unwrap());
    let fight = unsafe { core::str::from_utf8_unchecked(&fight) };
    bencher.iter(|| alphabet_war(black_box(fight)));
}
