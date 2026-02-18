//! <https://www.codewars.com/kata/5906436806d25f846400009b/train/rust>

use unchecked_std::prelude::*;

pub fn x(n: u32) -> String {
    assert!(!n.is_multiple_of(2));
    let mut res = String::with_capacity(
        usize::try_from(n as u64 * n as u64)
            .unwrap()
            .checked_mul("□".len())
            .unwrap()
            + n as usize, // addition does not overflow as tested below
    );
    unsafe {
        for i in 0..n / 2 {
            for _ in 0..i {
                res.push_unchecked('□');
            }
            res.push_unchecked('■');

            for _ in 0..n - 2 * (i + 1) {
                res.push_unchecked('□');
            }

            res.push_unchecked('■');
            for _ in 0..i {
                res.push_unchecked('□');
            }
            res.push_unchecked('\n');
        }

        for _ in 0..n / 2 {
            res.push_unchecked('□');
        }
        res.push_unchecked('■');
        for _ in 0..n / 2 {
            res.push_unchecked('□');
        }

        for i in (0..n / 2).rev() {
            res.push_unchecked('\n');
            for _ in 0..i {
                res.push_unchecked('□');
            }
            res.push_unchecked('■');

            for _ in 0..n - 2 * (i + 1) {
                res.push_unchecked('□');
            }

            res.push_unchecked('■');
            for _ in 0..i {
                res.push_unchecked('□');
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use num_integer::Roots;

    #[test]
    fn capacity_addition_should_not_overflow() {
        let n = (u64::MAX / 3).sqrt();
        assert!((3 * n * n).checked_add(n).is_some());

        let n = (u32::MAX / 3).sqrt();
        assert!((3 * n * n).checked_add(n).is_some());

        let n = (u16::MAX / 3).sqrt();
        assert!((3 * n * n).checked_add(n).is_some());
    }
}
