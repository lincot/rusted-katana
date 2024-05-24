#![feature(test)]

extern crate test;
use core::array;
use neutralisation::neutralise;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
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
