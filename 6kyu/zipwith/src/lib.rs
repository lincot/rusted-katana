//! <https://www.codewars.com/kata/5825792ada030e9601000782/train/rust>

pub fn zip_with<F, T, S, R>(f: F, a: &[T], b: &[S]) -> Vec<R>
where
    F: Fn(T, S) -> R,
    T: Copy,
    S: Copy,
{
    a.iter().zip(b).map(|(&x, &y)| f(x, y)).collect()
}
