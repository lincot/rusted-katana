#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

fn get_grid() -> [Vec<char>; 4] {
    [
        vec!['h', 'e', 'l', 'l'],
        vec!['o', 'c', 'o', 'd'],
        vec!['e', 'w', 'a', 'r'],
        vec!['r', 'i', 'o', 'r'],
    ]
}

const INDICES: [usize; 10] = [5, 7, 9, 3, 6, 6, 8, 8, 16, 13];

#[bench]
fn bench(bencher: &mut Bencher) {
    let grid = get_grid();
    let grid = black_box(&grid);
    let indices = black_box(&INDICES);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::grid_index(grid, indices));
        }
    })
}
