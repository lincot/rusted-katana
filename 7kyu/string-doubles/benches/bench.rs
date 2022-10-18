#![no_std]
#![feature(test)]

extern crate test;
use string_doubles::doubles;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| doubles(black_box("ххбннннныааааамвидхвыдуслььььиддхпй")));
}
