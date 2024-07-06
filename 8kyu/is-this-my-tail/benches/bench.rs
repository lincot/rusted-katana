#![feature(test)]

extern crate test;
use is_this_my_tail::correct_tail;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| correct_tail(black_box("Badger"), black_box('s')));
}
