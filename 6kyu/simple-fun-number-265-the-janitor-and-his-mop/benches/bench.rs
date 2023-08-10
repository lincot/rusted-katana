#![no_std]
#![feature(test)]

extern crate test;
use simple_fun_number_265_the_janitor_and_his_mop::the_janitor;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(the_janitor(black_box(
                "rkuhsxtflzvytbtwxyarsglibmhfmmikyolfmopbtkzxezjahq",
            )));
        }
    });
}
