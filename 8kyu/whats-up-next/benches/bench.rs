#![feature(test)]

extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};
use whats_up_next::next_item;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let slice: [_; if cfg!(miri) { 16 } else { 1024 }] =
        array::from_fn(|_| array::from_fn::<_, 20, _>(|_| rng.gen_range(b'a'..=b'z')));
    let slice: [_; if cfg!(miri) { 16 } else { 1024 }] =
        array::from_fn(|i| unsafe { core::str::from_utf8_unchecked(&slice[i]) });
    let find = slice[slice.len() / 2];
    bencher.iter(|| black_box(next_item(black_box(&slice), black_box(find))));
}
