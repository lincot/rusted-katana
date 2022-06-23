#![feature(test)]

extern crate test;
use find_the_first_non_consecutive_number::first_non_consecutive;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::iter::repeat;
use test::{black_box, Bencher};

fn get_arr(rng: &mut impl Rng) -> Vec<i32> {
    const SIZE: usize = 1000;
    const ANS_INDEX: usize = 300;
    const OFFSET: i32 = 4;
    (OFFSET..ANS_INDEX as i32 + OFFSET)
        .chain(repeat(()).map(|()| rng.gen()).take(SIZE - ANS_INDEX))
        .collect()
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::seed_from_u64(222);
    let arr = get_arr(&mut rng);
    let arr = black_box(&arr);
    bencher.iter(|| first_non_consecutive(arr));
}
