#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};
use write_number_in_expanded_form_part_2::expanded_form;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let nums: [_; if cfg!(miri) { 5 } else { 100 }] =
        array::from_fn(|_| rng.gen_range(1.0..=100_000_000_000.));
    bencher.iter(|| {
        for num in nums {
            black_box(expanded_form(black_box(num)));
        }
    });
}
