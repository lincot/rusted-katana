#![feature(test)]

extern crate test;
use moduli_number_system::from_nb_2str;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| from_nb_2str(black_box(3450), black_box(vec![13, 11, 7, 5, 3, 2])));
}
