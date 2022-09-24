//! <https://www.codewars.com/kata/5ae7e3f068e6445bc8000046/train/rust>

#![no_std]

const fn from_4_digits(d0: u16, d1: u16, d2: u16, d3: u16) -> u16 {
    1000 * d0 + 100 * d1 + 10 * d2 + d3
}

pub fn next_happy_year(year: u16) -> u16 {
    if year < 1023 {
        return 1023;
    }
    if 8975 < year {
        return 9012;
    }

    let year = year + 1;
    let mut d3 = year % 10;
    let mut d2 = year / 10 % 10;
    let mut d1 = year / 100 % 10;
    let mut d0 = year / 1000;

    if d0 == d1 {
        d1 += 1;
        d2 = 0;
        d3 = if d0 == 1 { 3 } else { 1 };
        return from_4_digits(d0, d1, d2, d3);
    }

    if d0 == d1 + 1 && d1 == d2 + 1 && d2 == d3 {
        d3 = d0 + 1;
        return from_4_digits(d0, d1, d2, d3);
    }

    for _ in 0..2 {
        if d0 == d2 {
            if d1 == 0 {
                d2 += 1;
                d3 = 1;
            } else {
                d2 = d0 + if d1 == d0 + 1 { 2 } else { 1 };
                d3 = 0;
            }
            break;
        }
        if d0 == d3 {
            d3 += 1;
        }

        if (d1 == 9 && d2 == 9) || (d1 == 9 && d2 == 8 && d3 == 8) {
            d0 += 1;
            d1 = 0;
            d2 = 1;
            d3 = if d0 == 2 { 3 } else { 2 };
            break;
        }

        if d1 == d2 {
            d2 += 1;
            d3 = 0;
        }
        if d1 == d3 {
            if d1 == 9 {
                d2 += 1;
                d3 = 0;
            } else {
                d3 += 1;
            }
        }

        if d2 == 9 && d3 == 9 {
            d1 += if d0 == d1 + 1 { 2 } else { 1 };
            d2 = 0;
            if d0 == 1 || d1 == 1 {
                if d0 == 2 || d1 == 2 {
                    d3 = 3;
                } else {
                    d3 = 2;
                }
            } else {
                d3 = 1;
            }
            break;
        }
        if d2 == d3 {
            d3 += 1;
        }
    }

    from_4_digits(d0, d1, d2, d3)
}
