#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};
use word_values::word_value;

const LEN: usize = if cfg!(miri) { 10 } else { 100 };

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let words: [_; LEN] = array::from_fn(|_| {
        let res = (0..rng.gen_range(0..100))
            .map(|_| rng.gen_range(b'a'..=b'z'))
            .collect();
        unsafe { String::from_utf8_unchecked(res) }
    });
    let words: [_; LEN] = array::from_fn(|i| words[i].as_str());
    bencher.iter(|| word_value(black_box(&words)));
}
