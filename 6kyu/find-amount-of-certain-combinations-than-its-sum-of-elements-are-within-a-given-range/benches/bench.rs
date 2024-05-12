#![feature(test)]
#![feature(slice_partition_dedup)]

extern crate test;
use core::array;
use find_amount_of_certain_combinations_than_its_sum_of_elements_are_within_a_given_range::find_comb_noncontig;
use rand::{seq::SliceRandom, Rng};
use rand_pcg::Pcg64;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let mut arr: [_; 16] = array::from_fn(|_| rng.gen_range(-100_000..100_000));
    arr.sort_unstable();
    let arr = arr.partition_dedup().0;
    arr.shuffle(&mut rng);
    bencher.iter(|| find_comb_noncontig(black_box(arr), black_box(0), black_box(100_000)));
}
