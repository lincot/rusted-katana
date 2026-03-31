#![feature(test)]

extern crate test;
use core::array;

use contamination_number_1_string::contamination;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let text: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|_| b'd');
    let text = unsafe { core::str::from_utf8_unchecked(&text) };
    bencher.iter(|| contamination(black_box(text), black_box("д")));
}
