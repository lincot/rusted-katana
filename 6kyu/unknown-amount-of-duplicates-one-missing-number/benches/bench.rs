#![feature(test)]

extern crate test;
use core::array;
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};
use unknown_amount_of_duplicates_one_missing_number::find_dups_miss;

#[bench]
fn bench(bencher: &mut Bencher) {
    const SIZE: usize = if cfg!(miri) { 128 } else { 8192 };
    const A: u32 = 1337;
    const B: u32 = A + SIZE as u32 * 2 / 3;
    const MISSING_I: u32 = (B - A) / 2;
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut arr: [_; SIZE] = array::from_fn(|i| {
        let i = i as u32;
        if i < B - A + 1 {
            if i == MISSING_I {
                A
            } else {
                A + i
            }
        } else if rng.gen() {
            rng.gen_range(A..A + MISSING_I)
        } else {
            rng.gen_range(A + MISSING_I + 1..B)
        }
    });
    arr.shuffle(&mut rng);
    bencher.iter(|| find_dups_miss(black_box(&arr)));
}
