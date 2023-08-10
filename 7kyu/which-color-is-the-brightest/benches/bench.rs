#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::string::String;
use core::array;
use lexical_core::{
    write_with_options_unchecked, FormattedSize, NumberFormatBuilder, WriteIntegerOptions,
};
use rand::Rng;
use rand_pcg::Pcg64;
use test::{black_box, Bencher};
use which_color_is_the_brightest::brightest;

#[bench]
fn bench(bencher: &mut Bencher) {
    const FORMAT: u128 = NumberFormatBuilder::hexadecimal();
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let colors: [_; 100] = array::from_fn(|_| {
        let mut s = [b'0'; 7];
        s[0] = b'#';
        let mut buf = [0; u32::FORMATTED_SIZE];
        unsafe {
            let written_len = write_with_options_unchecked::<_, FORMAT>(
                rng.gen_range(0..1u32 << 24),
                &mut buf,
                &WriteIntegerOptions::new(),
            )
            .len();
            s[7 - written_len..].copy_from_slice(&buf[..written_len]);
            String::from_utf8_unchecked(s.into())
        }
    });
    bencher.iter(|| brightest(black_box(&colors)));
}
