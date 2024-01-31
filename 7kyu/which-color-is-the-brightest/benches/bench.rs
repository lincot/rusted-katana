#![feature(test)]

extern crate test;
use core::array;
use rand::seq::SliceRandom;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};
use which_color_is_the_brightest::brightest;

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let colors: [_; 100] = array::from_fn(|_| {
        let mut s = [b'0'; 7];
        s[0] = b'#';
        for b in &mut s[1..] {
            *b = *[
                b'0', b'1', b'2', b'3', b'4', b'5', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C',
                b'D', b'E', b'F',
            ]
            .choose(&mut rng)
            .unwrap();
        }
        unsafe { String::from_utf8_unchecked(s.into()) }
    });
    bencher.iter(|| brightest(black_box(&colors)));
}
