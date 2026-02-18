#![feature(test)]

extern crate test;
use parse_bank_account_number::parse_bank_account;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        parse_bank_account(black_box(concat!(
            " _     _  _     _  _  _  _  _ \n",
            "| |  | _| _||_||_ |_   ||_||_|\n",
            "|_|  ||_  _|  | _||_|  ||_| _|\n"
        )))
    });
}
