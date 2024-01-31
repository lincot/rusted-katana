#![feature(test)]

extern crate test;
use beginner_lost_without_a_map::maps;
use core::array;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let values = array::from_fn::<_, 1024, _>(|i| 1337 * i as i32).to_vec();
    bencher.iter(|| maps(black_box(&values)));
}
