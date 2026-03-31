#![feature(test)]

extern crate test;
use core::array;

use back_and_forth_then_reverse::arrange;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s: [_; if cfg!(miri) { 64 } else { 1024 }] = array::from_fn(|i| i as i32);
    bencher.iter(|| arrange(black_box(&s)));
}
