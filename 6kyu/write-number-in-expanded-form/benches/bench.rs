#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use write_number_in_expanded_form::expanded_form;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| expanded_form(black_box(70304)));
}
