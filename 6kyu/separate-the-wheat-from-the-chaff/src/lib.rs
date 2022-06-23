//! <https://www.codewars.com/kata/5bdcd20478d24e664d00002c/train/rust>

pub fn wheat_from_chaff(xs: &[i32]) -> Vec<i32> {
    let mut xs = xs.to_vec();

    if xs.is_empty() {
        return xs;
    }

    let mut i = 0;
    let mut j = xs.len() - 1;
    while i < j {
        if i >= xs.len() {
            unsafe { core::hint::unreachable_unchecked() };
        }
        if xs[i] > 0 {
            while i < j {
                if j >= xs.len() {
                    unsafe { core::hint::unreachable_unchecked() };
                }
                if xs[j] < 0 {
                    xs.swap(i, j);
                    break;
                }

                j -= 1;
            }
        }

        i += 1;
    }

    xs
}
