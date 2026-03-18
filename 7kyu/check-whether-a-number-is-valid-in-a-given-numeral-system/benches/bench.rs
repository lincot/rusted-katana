#![feature(test)]

extern crate test;
use check_whether_a_number_is_valid_in_a_given_numeral_system::validate_base;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| validate_base(black_box("AB456CDEF"), black_box(16)));
}
