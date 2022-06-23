//! <https://www.codewars.com/kata/5af15a37de4c7f223e00012d/train/rust>

pub fn men_from_boys(xs: &[i16]) -> Vec<i16> {
    let mut boys = Vec::with_capacity(xs.len());
    let mut men = Vec::with_capacity(xs.len());

    for &x in xs {
        if x % 2 == 0 {
            boys.push(x);
        } else {
            men.push(x);
        }
    }

    boys.sort_unstable();
    boys.dedup();

    men.sort_unstable_by_key(|&v| std::cmp::Reverse(v));
    men.dedup();

    boys.extend(men);

    boys
}
