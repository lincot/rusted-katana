#![feature(test)]

extern crate test;
use get_character_from_ascii_value::get_char;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_char(black_box(b'a' as _)));
}
