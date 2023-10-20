#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use neutralisation::neutralise;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let [s1, s2] = array::from_fn(|_| {
        array::from_fn::<
            _,
            {
                if cfg!(miri) {
                    100
                } else {
                    10_000
                }
            },
            _,
        >(|_| if rng.gen() { b'+' } else { b'-' })
    });
    let [s1, s2] = [&s1, &s2].map(|x| unsafe { core::str::from_utf8_unchecked(x) });
    bencher.iter(|| neutralise(black_box(s1), black_box(s2)));
}
