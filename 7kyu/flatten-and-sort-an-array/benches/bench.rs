#![feature(test)]

extern crate test;
use core::array;
use flatten_and_sort_an_array::flatten_and_sort;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let arr: [_; 16] = array::from_fn(|_| {
        array::from_fn::<
            _,
            {
                if cfg!(miri) {
                    16
                } else {
                    1024
                }
            },
            _,
        >(|_| rng.gen())
        .into()
    });
    bencher.iter(|| flatten_and_sort(black_box(&arr)));
}
