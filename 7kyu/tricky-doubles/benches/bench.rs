#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use tricky_doubles::tricky_doubles;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(tricky_doubles(black_box(14_521_452)));
        black_box(tricky_doubles(black_box(14_521_451)));
        black_box(tricky_doubles(black_box(u32::MAX / 10)));
    });
}
