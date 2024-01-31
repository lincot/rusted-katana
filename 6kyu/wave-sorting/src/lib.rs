//! <https://www.codewars.com/kata/596f28fd9be8ebe6ec0000c1/train/rust>

pub fn wave_sort(xs: &mut [i32]) {
    for i in 1..xs.len() {
        if i % 2 == 0 && xs[i] < xs[i - 1] || i % 2 == 1 && xs[i - 1] < xs[i] {
            xs.swap(i - 1, i);
        }
    }
}
