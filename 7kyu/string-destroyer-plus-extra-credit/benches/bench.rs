#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use string_destroyer_plus_extra_credit::destroy;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input_sets = black_box(vec![['B', 'b'].into(), ['C', 'm', 'f'].into()]);
    bencher.iter(|| destroy(input_sets.clone()));
}
