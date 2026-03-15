//! <https://www.codewars.com/kata/5ecc1d68c6029000017d8aaf/train/rust>

use core::mem::MaybeUninit;

pub fn max_hexagon_beam(n: u8, seq: &[i32]) -> i32 {
    let n = n as isize;
    let mut seq = seq.iter().cycle();
    let size = 2 * n as usize - 1;
    let mut sums = [MaybeUninit::<i32>::uninit(); 3 * (2 * u8::MAX as usize - 1)];
    let sums = unsafe {
        let slice = sums.get_unchecked_mut(..3 * size);
        slice.fill(MaybeUninit::new(0i32));
        slice.assume_init_mut()
    };

    for q in (-n + 1)..n {
        for r in ((-n).max(-q - n) + 1)..n.min(-q + n) {
            let v = *seq.next().unwrap();
            for idx in [
                (q + n - 1) as usize,
                size + (r + n - 1) as usize,
                2 * size + (-q - r + n - 1) as usize,
            ] {
                unsafe {
                    *sums.get_unchecked_mut(idx) += v;
                }
            }
        }
    }

    *sums.iter().max().unwrap()
}
