#![feature(test)]

extern crate test;
use count_of_positives_slash_sum_of_negatives::count_positives_sum_negatives;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::iter::repeat;
use test::{black_box, Bencher};

fn get_input(rng: &mut impl Rng) -> Vec<i32> {
    const SIZE: usize = 10000;
    repeat(()).map(|()| rng.gen()).take(SIZE).collect()
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::seed_from_u64(222);
    let input = get_input(&mut rng);
    let input = black_box(&input);
    bencher.iter(|| count_positives_sum_negatives(input.clone()));
}
