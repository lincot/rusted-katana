#![feature(test)]

extern crate test;
use core::array;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;
use street_fighter_2_character_selection::{Direction, street_fighter_selection};
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let moves = array::from_fn::<_, { if cfg!(miri) { 50 } else { 1000 } }, _>(|_| {
        match rng.random_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        }
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
