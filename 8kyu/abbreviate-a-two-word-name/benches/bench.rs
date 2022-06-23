#![feature(test)]

extern crate test;
use abbreviate_a_two_word_name::abbrev_name;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let name = black_box("Дмитрий Муратов");
    bencher.iter(|| abbrev_name(name));
}
