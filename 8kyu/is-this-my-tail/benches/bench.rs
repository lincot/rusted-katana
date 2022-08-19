#![feature(test)]

extern crate test;
use is_this_my_tail::correct_tail;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let body = black_box("Badger");
    let tail = black_box('s');
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(correct_tail(body, tail));
        }
    });
}
