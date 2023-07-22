#![no_std]
#![feature(test)]

extern crate test;
use core::array;
use switcheroo::switcheroo;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s: [_; 1024] = array::from_fn(|i| b"abc"[i % 3]);
    bencher.iter(|| switcheroo(black_box(unsafe { core::str::from_utf8_unchecked(&s) })));
}
