//! <https://www.codewars.com/kata/5f5802bf4c2cc4001a6f859e/train/rust>

pub fn grid_index(grid: &[Vec<char>], indices: &[usize]) -> String {
    let size = grid.len();
    assert!(size != 0);

    indices
        .iter()
        .map(|i| i - 1)
        .map(|i| grid[i / size][i % size])
        .collect()
}
