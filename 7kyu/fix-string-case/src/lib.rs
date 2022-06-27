//! <https://www.codewars.com/kata/5b180e9fedaa564a7000009a/train/rust>

pub fn solve(s: &str) -> String {
    let chars: Box<_> = s
        .chars()
        .map(|c| (c, c.is_lowercase(), c.is_uppercase()))
        .collect();

    let lowercase_count = chars.iter().filter(|&&(_, l, _)| l).count();
    let uppercase_count = chars.iter().filter(|&&(_, _, u)| u).count();

    // usually same len
    let mut res = String::with_capacity(s.len());

    if uppercase_count > lowercase_count {
        for &(c, _, u) in chars.iter() {
            if u {
                res.push(c);
            } else {
                res.extend(c.to_uppercase());
            }
        }
    } else {
        for &(c, l, _) in chars.iter() {
            if l {
                res.push(c);
            } else {
                res.extend(c.to_lowercase());
            }
        }
    }

    res
}
