#![feature(test)]

extern crate test;
use core::array;
use maximum_length_difference::mx_dif_lg;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let [a1, a2]: [[_; 100]; 2] = array::from_fn(|_| {
        array::from_fn(|_| {
            let len = rng.gen_range(0..1000);
            let mut s = String::with_capacity(len);
            unsafe { s.as_mut_vec().set_len(len) };
            s
        })
    });
    let a1: [_; 100] = array::from_fn(|i| a1[i].as_str());
    let a2: [_; 100] = array::from_fn(|i| a2[i].as_str());
    bencher.iter(|| mx_dif_lg(black_box(a1.into()), black_box(a2.into())));
}
