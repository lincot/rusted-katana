#![feature(test)]

extern crate test;
use string_destroyer_plus_extra_credit::destroy;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| destroy(black_box(vec![['B', 'b'].into(), ['C', 'm', 'f'].into()])));
}
