#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use the_if_function::_if;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| _if(black_box(true), black_box(|| 1), black_box(|| 2)));
}
