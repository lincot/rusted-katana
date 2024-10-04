#![feature(test)]

extern crate test;
use score_from_permutations_of_combinations_of_an_integer::sc_perm_comb;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sc_perm_comb(black_box(if cfg!(miri) { 128 } else { 128_752 })));
}
