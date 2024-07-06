#![feature(test)]

extern crate test;
use convert_string_to_camel_case::to_camel_case;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| to_camel_case(black_box("Самый-скрытный-воин")));
}
