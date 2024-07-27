#![feature(test)]

extern crate test;
use core::array;
use count_the_photos::count_photos;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let road: [_; 1024] = array::from_fn(|_| *b">.<".choose(&mut rng).unwrap());
    let road = unsafe { core::str::from_utf8_unchecked(&road) };
    bencher.iter(|| count_photos(black_box(road)));
}
