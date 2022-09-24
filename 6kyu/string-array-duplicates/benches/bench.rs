#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use string_array_duplicates::dup;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arry = black_box(vec![
        "adanac".into(),
        "soonness".into(),
        "toolless".into(),
        "ppellee".into(),
    ]);
    bencher.iter(|| dup(arry.clone()));
}
