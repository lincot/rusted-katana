#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use help_the_bookseller::stock_list;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(vec![
        "BKWRKAE 125",
        "ROB 530",
        "DRTYMC 060",
        "BTSQZB 239",
        "CBART 20",
        "BKWRKAD 125",
        "RHODODEC 123",
    ]);
    let b = black_box(vec!["A", "X", "U"]);
    bencher.iter(|| stock_list(a.clone(), b.clone()));
}
