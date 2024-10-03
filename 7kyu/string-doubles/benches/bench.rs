#![feature(test)]

extern crate test;
use string_doubles::doubles;
use test::{black_box, Bencher};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| doubles(black_box("xxbnnnnnyaaaaamrrrmooomqqqqjabbbzz")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| doubles(black_box("ххбннннныааааамвидхвыдуслььььиддхпй")));
}
