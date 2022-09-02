#![feature(test)]

extern crate test;
use maximum_length_difference::mx_dif_lg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a1 = black_box(vec![
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
    let a2 = black_box(vec!["cccooommaaqqoxii", "gggqaffhhh", "tttoowwwmmww"]);
    bencher.iter(|| mx_dif_lg(a1.clone(), a2.clone()));
}
