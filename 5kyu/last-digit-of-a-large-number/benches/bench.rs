#![no_std]
#![feature(test)]

extern crate test;
use last_digit_of_a_large_number::last_digit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let str1_0 = black_box("1371");
    let str1_1 = black_box("1371");
    let str1_2 = black_box("1372");
    let str1_3 = black_box("1373");
    let str1_4 = black_box("1374");
    let str1_5 = black_box("1375");
    let str1_6 = black_box("1376");
    let str1_7 = black_box("1377");
    let str1_8 = black_box("1378");
    let str1_9 = black_box("1379");
    let str2_0 = black_box("97376");
    let str2_1 = black_box("97376");
    let str2_2 = black_box("97376");
    let str2_3 = black_box("97376");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(last_digit(str1_0, str2_0));
            black_box(last_digit(str1_1, str2_0));
            black_box(last_digit(str1_2, str2_0));
            black_box(last_digit(str1_3, str2_0));
            black_box(last_digit(str1_4, str2_0));
            black_box(last_digit(str1_5, str2_0));
            black_box(last_digit(str1_6, str2_0));
            black_box(last_digit(str1_7, str2_0));
            black_box(last_digit(str1_8, str2_0));
            black_box(last_digit(str1_9, str2_0));
            black_box(last_digit(str1_0, str2_1));
            black_box(last_digit(str1_1, str2_1));
            black_box(last_digit(str1_2, str2_1));
            black_box(last_digit(str1_3, str2_1));
            black_box(last_digit(str1_4, str2_1));
            black_box(last_digit(str1_5, str2_1));
            black_box(last_digit(str1_6, str2_1));
            black_box(last_digit(str1_7, str2_1));
            black_box(last_digit(str1_8, str2_1));
            black_box(last_digit(str1_9, str2_1));
            black_box(last_digit(str1_0, str2_2));
            black_box(last_digit(str1_1, str2_2));
            black_box(last_digit(str1_2, str2_2));
            black_box(last_digit(str1_3, str2_2));
            black_box(last_digit(str1_4, str2_2));
            black_box(last_digit(str1_5, str2_2));
            black_box(last_digit(str1_6, str2_2));
            black_box(last_digit(str1_7, str2_2));
            black_box(last_digit(str1_8, str2_2));
            black_box(last_digit(str1_9, str2_2));
            black_box(last_digit(str1_0, str2_3));
            black_box(last_digit(str1_1, str2_3));
            black_box(last_digit(str1_2, str2_3));
            black_box(last_digit(str1_3, str2_3));
            black_box(last_digit(str1_4, str2_3));
            black_box(last_digit(str1_5, str2_3));
            black_box(last_digit(str1_6, str2_3));
            black_box(last_digit(str1_7, str2_3));
            black_box(last_digit(str1_8, str2_3));
            black_box(last_digit(str1_9, str2_3));
        }
    });
}
