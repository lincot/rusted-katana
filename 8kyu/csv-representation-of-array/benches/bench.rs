#![feature(test)]

extern crate test;
use core::array;
use csv_representation_of_array::to_csv_text;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const SIZE: usize = if cfg!(miri) { 5 } else { 30 };
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let array: [_; SIZE] = array::from_fn(|_| array::from_fn::<_, SIZE, _>(|_| rng.gen()).into());
    bencher.iter(|| to_csv_text(black_box(&array)));
}
