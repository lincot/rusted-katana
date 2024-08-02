#![feature(test)]

extern crate test;
use core::array;
use millipede_of_words::millipede;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

const LEN: usize = 7;

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let words: [_; LEN] = array::from_fn(|_| unsafe {
        String::from_utf8_unchecked(vec![rng.gen_range(b'a'..=b'z'), rng.gen_range(b'a'..=b'z')])
    });
    let words: [_; LEN] = array::from_fn(|i| words[i].as_str());
    bencher.iter(|| millipede(black_box(&words)));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let words: [_; LEN] = array::from_fn(|_| {
        let mut s = String::with_capacity(8);
        s.push(*UKRAINIAN_ALPHABET.choose(&mut rng).unwrap());
        s.push(*UKRAINIAN_ALPHABET.choose(&mut rng).unwrap());
        s
    });
    let words: [_; LEN] = array::from_fn(|i| words[i].as_str());
    bencher.iter(|| millipede(black_box(&words)));
}

const UKRAINIAN_ALPHABET: &[char; 33] = &[
    'а', 'б', 'в', 'г', 'ґ', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к', 'л', 'м', 'н', 'о',
    'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь', 'ю', 'я',
];
