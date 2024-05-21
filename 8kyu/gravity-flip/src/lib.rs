//! <https://www.codewars.com/kata/5f70c883e10f9e0001c89673/train/rust>

pub fn flip(dir: char, cubes: &[u32]) -> Vec<u32> {
    let mut cubes = cubes.to_vec();
    if dir == 'R' {
        vqsort_rs::sort(&mut cubes);
    } else {
        vqsort_rs::sort_descending(&mut cubes);
    }
    cubes
}
