#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use core::array;
use rand::Rng;
use rand_pcg::Pcg64;
use street_fighter_2_character_selection::{street_fighter_selection, Direction};
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64::new(
        0xcafe_f00d_d15e_a5e5,
        0x0a02_bdbf_7bb3_c0a7_ac28_fa16_a64a_bf96,
    );
    let moves = array::from_fn::<_, 1000, _>(|_| match rng.gen_range(0..4) {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        3 => Direction::Right,
        _ => unreachable!(),
    });
    bencher.iter(|| {
        street_fighter_selection(
            black_box(&[
                ["Ryu", "E.Honda", "Blanka", "Guile", "Balrog", "Vega"],
                ["Ken", "Chun Li", "Zangief", "Dhalsim", "Sagat", "M.Bison"],
            ]),
            black_box(&[0, 0]),
            black_box(&moves),
        )
    });
}
