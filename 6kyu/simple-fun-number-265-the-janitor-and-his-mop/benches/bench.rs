#![feature(test)]

extern crate test;
use simple_fun_number_265_the_janitor_and_his_mop::the_janitor;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        the_janitor(black_box(
            "rkuhsxtflzvytbtwxyarsglibmhfmmikyolfmopbtkzxezjahq",
        ))
    });
}
