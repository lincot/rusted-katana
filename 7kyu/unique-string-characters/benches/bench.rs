#![feature(test)]

extern crate test;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};
use unique_string_characters::solve;

const LEN: usize = if cfg!(miri) { 10 } else { 1000 };

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let a: String = (0..LEN)
        .map(|_| UKRAINIAN_ALPHABET[..20].choose(&mut rng).unwrap())
        .collect();
    let b: String = (0..LEN)
        .map(|_| {
            UKRAINIAN_ALPHABET[UKRAINIAN_ALPHABET.len() - 20..]
                .choose(&mut rng)
                .unwrap()
        })
        .collect();
    bencher.iter(|| solve(black_box(&a), black_box(&b)));
}

const UKRAINIAN_ALPHABET: &[char; 33] = &[
    'а', 'б', 'в', 'г', 'ґ', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к', 'л', 'м', 'н', 'о',
    'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь', 'ю', 'я',
];
