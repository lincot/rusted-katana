#![feature(test)]

extern crate test;
use help_the_bookseller::stock_list;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        stock_list(
            black_box(vec![
                "BKWRKAE 125",
                "ROB 530",
                "DRTYMC 060",
                "BTSQZB 239",
                "CBART 20",
                "BKWRKAD 125",
                "RHODODEC 123",
            ]),
            black_box(vec!["A", "X", "U"]),
        )
    });
}
