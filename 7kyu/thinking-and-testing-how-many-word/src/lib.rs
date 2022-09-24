//! <https://www.codewars.com/kata/56eff1e64794404a720002d2/train/rust>

#![no_std]

pub fn testit(s: &str) -> usize {
    let s = s.as_bytes();

    let mut res = 0;

    let mut d_ = 0;
    while let Some(w) = unsafe { s.get_unchecked(d_..) }
        .iter()
        .position(|b| [b'w', b'W'].contains(b))
        .map(|pos| pos + d_)
    {
        if let Some(o) = unsafe { s.get_unchecked(w + 1..) }
            .iter()
            .position(|b| [b'o', b'O'].contains(b))
            .map(|pos| pos + w + 1)
        {
            if let Some(r) = unsafe { s.get_unchecked(o + 1..) }
                .iter()
                .position(|b| [b'r', b'R'].contains(b))
                .map(|pos| pos + o + 1)
            {
                if let Some(d) = unsafe { s.get_unchecked(r + 1..) }
                    .iter()
                    .position(|b| [b'd', b'D'].contains(b))
                    .map(|pos| pos + r + 1)
                {
                    res += 1;
                    d_ = d + 1;
                } else {
                    break;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    res
}
