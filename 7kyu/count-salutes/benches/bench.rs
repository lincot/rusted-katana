#![feature(test)]

extern crate test;
use count_salutes::count_salutes;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_salutes(black_box(">>>>>>>>>>>>>>>>>>>>>----<->")));
}
