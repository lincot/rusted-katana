#![feature(test)]

extern crate test;
use rand::prelude::*;
use rand_pcg::Pcg64Mcg;
use skills_master::count_skills;
use std::collections::{HashMap, HashSet};
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let tree = make_tree(if cfg!(miri) { 30 } else { 300_000 });
    let required = (0..tree.len() / 2)
        .map(|_| rng.gen_range(0..tree.len()))
        .collect();
    bencher.iter(|| count_skills(black_box(&tree), black_box(&required)));
}

// taken from the kata tests
fn make_tree(size: usize) -> Vec<usize> {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut tree_map = HashMap::<usize, usize>::from([(0, 0)]);
    let mut bag = HashSet::from([0]);
    let mut indices: Vec<_> = (1..size).collect();
    indices.shuffle(&mut rng);
    while !indices.is_empty() {
        let mut temp = HashSet::new();
        for parent in bag {
            if indices.is_empty() {
                break;
            }
            let n = rng.gen_range(1..=indices.len().min(5));
            let children = &indices[indices.len() - n..];
            for child in children {
                temp.insert(*child);
                tree_map.insert(*child, parent);
            }
            indices.truncate(indices.len() - n);
        }
        bag = temp;
    }
    (0..size).map(|i| *tree_map.get(&i).unwrap()).collect()
}
