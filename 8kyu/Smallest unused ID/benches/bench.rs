#![feature(test)]

extern crate test;
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::iter::repeat;
use test::{black_box, Bencher};

fn get_ids(rng: &mut impl Rng) -> Vec<usize> {
    const SIZE: usize = 1000;
    const ANS: usize = 300;

    let mut ids = (0..ANS)
        .chain((0..ANS).step_by(2))
        .chain(
            repeat(())
                .map(|()| rng.gen())
                .take(SIZE - ANS - (ANS + 1) / 2),
        )
        .collect::<Vec<usize>>();

    ids.shuffle(rng);
    ids
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::seed_from_u64(222);
    let ids = get_ids(&mut rng);
    let ids = black_box(&ids);
    bencher.iter(|| solution::next_id(ids))
}
