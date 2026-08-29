#![feature(test)]

extern crate test;
use sum_it_continuously::add;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let list = [5; 1000];
    bencher.iter(|| add(black_box(&list)));
}
