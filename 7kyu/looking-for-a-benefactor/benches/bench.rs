#![feature(test)]

extern crate test;
use core::array;
use looking_for_a_benefactor::new_avg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr: [_; 16] = array::from_fn(|i| i as f64 * 100.);
    bencher.iter(|| new_avg(black_box(&arr), 4800.));
}
