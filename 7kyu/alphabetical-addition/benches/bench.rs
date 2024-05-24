#![feature(test)]

extern crate test;
use alphabetical_addition::add_letters;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let letters: [_; 1024] = array::from_fn(|_| rng.gen_range(b'a'..=b'z') as char);
    bencher.iter(|| add_letters(black_box(letters.into())));
}
