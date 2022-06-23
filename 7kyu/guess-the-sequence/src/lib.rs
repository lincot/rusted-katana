//! <https://www.codewars.com/kata/5b45e4b3f41dd36bf9000090/train/rust>

pub fn sequence(x: u8) -> Vec<u8> {
    if x < 10 {
        return (1..=x).collect();
    }

    let mut res = Vec::with_capacity(x as _);

    for d in 1..=9 {
        res.push(d);
        if x >= 10 * d {
            res.extend(10 * d..=(10 * d + 9).min(x));
        }
    }

    if x == 100 {
        res.insert(2, 100);
    }

    res
}
