#![feature(test)]

extern crate test;
use core::array;
use delete_occurrences_of_an_element_if_it_occurs_more_than_n_times::delete_nth;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let lst: [_; 1024] = array::from_fn(|_| rng.gen());
    bencher.iter(|| delete_nth(black_box(&lst), black_box(3)));
}
