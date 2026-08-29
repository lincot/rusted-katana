#![feature(test)]

extern crate test;
use core::array;

use find_the_unique_string::find_uniq;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    const COMMON_SET: &str = "ф@пкгхвыяomg,цид$^&";
    const UNIQ_SET: &str = "abcde";
    const LEN: usize = if cfg!(miri) { 15 } else { 1023 };

    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut form_str = |set: &str| -> String {
        let mut res = Vec::new();
        for ch in set.chars() {
            for _ in 0..rng.random_range(1..10) {
                let ch = if rng.random() {
                    ch.to_uppercase().next().unwrap()
                } else {
                    ch
                };
                res.push(ch);
            }
        }
        res.iter().collect()
    };
    let mut arr: [_; LEN] = array::from_fn(|_| form_str(COMMON_SET));
    arr[arr.len() / 2] = form_str(UNIQ_SET);
    let arr: [_; LEN] = array::from_fn(|i| arr[i].as_str());

    bencher.iter(|| find_uniq(black_box(&arr)));
}
