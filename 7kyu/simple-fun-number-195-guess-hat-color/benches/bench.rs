#![feature(test)]

extern crate test;
use simple_fun_number_195_guess_hat_color::{
    Color::{Black, White},
    guess_hat_color,
};
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        guess_hat_color(
            black_box(Black),
            black_box(White),
            black_box(Black),
            black_box(White),
        )
    });
}
