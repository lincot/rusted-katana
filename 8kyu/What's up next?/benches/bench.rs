#![feature(test)]

extern crate test;
use rand::{distributions::Alphanumeric, seq::SliceRandom, Rng, SeedableRng};
use rand_pcg::Pcg64Mcg;
use std::iter::repeat;
use test::{black_box, Bencher};

fn get_slice(rng: &mut impl Rng) -> Vec<String> {
    const SIZE: usize = 10;
    const S_SIZE: usize = 300;
    repeat(())
        .map(|()| {
            repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .map(char::from)
                .take(S_SIZE)
                .collect::<String>()
        })
        .take(SIZE)
        .collect()
}

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::seed_from_u64(222);
    let slice = get_slice(&mut rng);
    let find = slice.choose(&mut rng).unwrap();
    let slice = black_box(&slice);
    let find = black_box(find);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::next_item(slice, find.clone()));
        }
    });
}
