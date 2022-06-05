#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a1 = black_box(&[
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
    ]);
    let a2 = black_box(&["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"]);
    bencher.iter(|| solution::mx_dif_lg(a1.to_vec(), a2.to_vec()));
}
