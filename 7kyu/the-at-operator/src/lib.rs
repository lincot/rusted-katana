//! <https://www.codewars.com/kata/631f0c3a0b9cb0de6ded0529/train/rust>

pub fn evaluate(equation: String) -> Option<i64> {
    let equation = equation.as_bytes();
    let mut space_index = equation.iter().position(|&x| x == b' ').unwrap();
    let mut res: i64 = unsafe { core::str::from_utf8_unchecked(&equation[..space_index]) }
        .parse()
        .unwrap();
    loop {
        let cur_int_index = space_index + 3;
        let (last, si) = equation[cur_int_index..]
            .iter()
            .position(|&x| x == b' ')
            .map_or((true, equation.len()), |i| (false, cur_int_index + i));
        space_index = si;
        res = at(
            res,
            unsafe {
                core::str::from_utf8_unchecked(equation.get_unchecked(cur_int_index..space_index))
            }
            .parse()
            .unwrap(),
        )?;
        if last {
            break;
        }
    }
    Some(res)
}

fn at(a: i64, b: i64) -> Option<i64> {
    Some(a * (b + 2) + a.checked_div(b)?)
}
