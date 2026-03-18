#![feature(test)]

extern crate test;
use powers_of_i::pofi;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pofi(black_box(10)));
}
