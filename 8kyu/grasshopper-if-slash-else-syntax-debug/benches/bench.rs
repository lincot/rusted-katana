#![feature(test)]

extern crate test;
use grasshopper_if_slash_else_syntax_debug::check_alive;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| check_alive(black_box(55)));
}
