#![feature(test)]

extern crate test;
use build_a_pile_of_cubes::find_nb;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_nb(black_box(26_825_883_955_641)));
}
