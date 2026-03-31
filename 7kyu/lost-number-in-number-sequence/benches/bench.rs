#![feature(test)]

extern crate test;
use core::array;

use lost_number_in_number_sequence::find_deleted_number;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64Mcg;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let list: [_; 128] = array::from_fn(|i| i as u16);
    let mut mixed_list: [_; 127] = array::from_fn(|i| {
        if i < list.len() / 2 {
            list[i]
        } else {
            list[i + 1]
        }
    });
    mixed_list.shuffle(&mut rng);
    bencher.iter(|| find_deleted_number(black_box(&list), black_box(&mixed_list)));
}
