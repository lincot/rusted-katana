#![feature(test)]

extern crate test;
use abbreviate_a_two_word_name::abbrev_name;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| abbrev_name(black_box("Дмитрий Муратов")));
}
