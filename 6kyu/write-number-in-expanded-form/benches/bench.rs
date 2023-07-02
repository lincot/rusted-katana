#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use write_number_in_expanded_form::expanded_form;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| expanded_form(black_box(70304)));
}
