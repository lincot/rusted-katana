#![feature(test)]

extern crate test;
use fixme_replace_all_dots::replace_dots;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("one.two.three.one.two.three.one.two.three.one.two.three.one.two.three");
    bencher.iter(|| replace_dots(s));
}
