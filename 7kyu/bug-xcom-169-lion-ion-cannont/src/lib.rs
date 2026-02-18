//! <https://www.codewars.com/kata/6901ed1b24add08e1069f3d5/train/rust>

pub fn release_ions<F>(count: usize, mut dispenser: F)
where
    F: FnMut(),
{
    for _ in 0..count {
        dispenser();
    }
}
