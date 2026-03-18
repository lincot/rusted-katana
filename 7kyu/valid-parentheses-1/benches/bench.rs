#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use valid_parentheses_1::valid_parentheses;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| valid_parentheses(black_box("()(())((()))(())()")));
}
