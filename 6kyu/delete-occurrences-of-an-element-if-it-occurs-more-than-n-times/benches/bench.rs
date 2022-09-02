#![feature(test)]

extern crate test;
use delete_occurrences_of_an_element_if_it_occurs_more_than_n_times::delete_nth;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let lst = black_box(&[1, 1, 3, 3, 7, 2, 2, 2, 2]);
    let n = black_box(3);
    bencher.iter(|| delete_nth(lst, n));
}
