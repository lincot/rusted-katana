#![feature(test)]

extern crate test;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::iter::repeat;
use sum_of_positive::positive_sum;
use test::{black_box, Bencher};

fn get_slice(rng: &mut impl Rng) -> Box<[i32]> {
    const SIZE: usize = 10000;
    repeat(()).map(|()| rng.gen()).take(SIZE).collect()
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::seed_from_u64(222);
    let slice = get_slice(&mut rng);
    let slice = black_box(&slice);
    bencher.iter(|| positive_sum(slice));
}
