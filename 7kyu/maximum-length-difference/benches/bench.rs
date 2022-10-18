#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use maximum_length_difference::mx_dif_lg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        mx_dif_lg(
            black_box(vec![
                "hoqq",
                "bbllkw",
                "oox",
                "ejjuyyy",
                "plmiis",
                "xxxzgpsssa",
                "xxwwkktt",
                "znnnnfqknaz",
                "qqquuhii",
                "dvvvwz",
            ]),
            black_box(vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"]),
        )
    });
}
