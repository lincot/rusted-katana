#![feature(test)]

extern crate test;
use convert_boolean_values_to_strings_yes_or_no::bool_to_word;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(bool_to_word(black_box(true)));
        black_box(bool_to_word(black_box(false)));
    });
}
