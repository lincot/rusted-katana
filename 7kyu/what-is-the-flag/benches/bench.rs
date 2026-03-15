#![feature(test)]

extern crate test;
use test::Bencher;
use what_is_the_flag::flag;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(flag);
}
