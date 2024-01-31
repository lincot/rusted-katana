#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use switch_on_the_gravity::switch_gravity;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let lst: [_; if cfg!(miri) { 10 } else { 100 }] = array::from_fn(|_| {
        array::from_fn::<
            _,
            {
                if cfg!(miri) {
                    10
                } else {
                    100
                }
            },
            _,
        >(|_| if rng.gen() { '#' } else { '-' })
        .into()
    });
    bencher.iter(|| switch_gravity(black_box(&lst)));
}
