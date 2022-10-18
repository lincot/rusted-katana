#![no_std]
#![feature(test)]

extern crate test;
use is_this_my_tail::correct_tail;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(correct_tail(black_box("Badger"), black_box('s')));
        }
    });
}
