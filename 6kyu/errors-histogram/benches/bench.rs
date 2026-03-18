#![feature(test)]

extern crate test;
use errors_histogram::hist;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const S: &str = "tpwaemuqxdmwqbqrjbeosjnejqorxdozsxnrgpgqkeihqwybzyymqeazfkyiucesxwutgszbenzvgxibxrlvmzihcb";

    bencher.iter(|| hist(black_box(S)));
}
