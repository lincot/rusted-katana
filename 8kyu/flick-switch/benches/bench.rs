#![feature(test)]

extern crate test;
use core::array;

use flick_switch::flick_switch;
use rand::seq::IndexedRandom;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let list: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|_| {
        *[
            "bicycle",
            "jarmony",
            "flick",
            "sheep",
            "flick",
            "codewars",
            "flick",
            "code",
            "wars",
            "flick",
            "chocolate",
            "adventure",
            "sunshine",
        ]
        .choose(&mut rng)
        .unwrap()
    });
    bencher.iter(|| flick_switch(black_box(&list)));
}
