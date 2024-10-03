#![feature(test)]

extern crate test;
use abbreviate_a_two_word_name::abbrev_name;
use test::{black_box, Bencher};



#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| abbrev_name(black_box("Dmitry Muratov")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| abbrev_name(black_box("Дмитрий Муратов")));
}
