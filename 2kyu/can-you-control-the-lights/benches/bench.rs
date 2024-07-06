#![feature(test)]

extern crate test;
use can_you_control_the_lights::LightController;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

const N: usize = if cfg!(miri) { 10 } else { 50 };
const M: usize = if cfg!(miri) { 15 } else { 100 };
const N_SETS: usize = if cfg!(miri) { 1 } else { 50 };

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);

    let corresponding_lights_list = random_subsets(&mut rng, N, M);
    let lights_sets = random_subsets(&mut rng, N, N_SETS);

    bencher.iter(|| {
        let solver = LightController::new(black_box(N), black_box(&corresponding_lights_list));
        for lights_subset in &lights_sets {
            black_box(solver.solve(black_box(lights_subset)));
        }
    });
}

fn random_subsets(rng: &mut impl Rng, set_size: usize, subset_count: usize) -> Vec<Vec<usize>> {
    (0..subset_count)
        .map(|_| {
            let size = rng.gen_range(0..=set_size);
            random_subset(rng, set_size, size)
        })
        .collect()
}

fn random_subset(rng: &mut impl Rng, set_size: usize, subset_size: usize) -> Vec<usize> {
    let mut set: Vec<usize> = (0..set_size).collect();
    set.shuffle(rng);
    let mut subset = set[..subset_size].to_vec();
    subset.sort_unstable();
    subset
}
