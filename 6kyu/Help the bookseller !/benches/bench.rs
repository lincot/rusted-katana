#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(&[
        "BKWRKAE 125",
        "ROB 530",
        "DRTYMC 060",
        "BTSQZB 239",
        "CBART 20",
        "BKWRKAD 125",
        "RHODODEC 123",
    ]);
    let b = black_box(&["A", "X", "U"]);
    bencher.iter(|| solution::stock_list(a.to_vec(), b.to_vec()))
}
