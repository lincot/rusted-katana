#![feature(test)]

extern crate test;
use all_star_code_challenge_number_15_text_scrolling_machine::rotate;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| rotate(black_box("Hello world!")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| rotate(black_box("Привет всем!")));
}
