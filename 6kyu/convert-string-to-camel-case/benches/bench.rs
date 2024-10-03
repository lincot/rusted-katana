#![feature(test)]

extern crate test;
use convert_string_to_camel_case::to_camel_case;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| to_camel_case(black_box("The-Stealth-Warrior")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| to_camel_case(black_box("Самый-скрытный-воин")));
}
