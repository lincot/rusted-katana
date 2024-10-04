#![feature(test)]

extern crate test;
use core::array;
use merging_vectors_without_reallocation::merge;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const LEN_XS: usize = if cfg!(miri) { 10 } else { 1000 };
    const LEN_YS: usize = if cfg!(miri) { 20 } else { 2000 };

    let xs = array::from_fn::<_, LEN_XS, _>(|i| 1337 * i).to_vec();
    let ys = array::from_fn::<_, LEN_YS, _>(|i| 7331 * i).to_vec();
    bencher.iter(|| merge(black_box(&xs), black_box(&ys)));
}
