#![no_std]
#![feature(test)]

extern crate test;
use delete_occurrences_of_an_element_if_it_occurs_more_than_n_times::delete_nth;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| delete_nth(black_box(&[1, 1, 3, 3, 7, 2, 2, 2, 2]), black_box(3)));
}
