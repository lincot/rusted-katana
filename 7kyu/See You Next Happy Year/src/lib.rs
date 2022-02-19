//! <https://www.codewars.com/kata/5ae7e3f068e6445bc8000046/train/rust>

const fn from_4_digits(d1: u16, d2: u16, d3: u16, d4: u16) -> u16 {
    1000 * d1 + 100 * d2 + 10 * d3 + d4
}

pub fn next_happy_year(mut year: u16) -> u16 {
    if year < 1023 {
        return 1023;
    }
    if 8975 < year {
        return 9012;
    }

    year += 1;
    let mut d4 = year % 10;
    year /= 10;
    let mut d3 = year % 10;
    year /= 10;
    let mut d2 = year % 10;
    year /= 10;
    let mut d1 = year % 10;

    if d1 == d2 {
        d2 += 1;
        d3 = 0;
        d4 = if d1 == 1 { 3 } else { 1 };
        return from_4_digits(d1, d2, d3, d4);
    }

    if d1 == d2 + 1 && d2 == d3 + 1 && d3 == d4 {
        d4 = d1 + 1;
        return from_4_digits(d1, d2, d3, d4);
    }

    for _ in 0..2 {
        if d1 == d3 {
            if d2 == 0 {
                d3 += 1;
                d4 = 1;
            } else {
                d3 = d1 + if d2 == d1 + 1 { 2 } else { 1 };
                d4 = 0;
            }
            break;
        }
        if d1 == d4 {
            d4 += 1;
        }

        if (d2 == 9 && d3 == 9) || (d2 == 9 && d3 == 8 && d4 == 8) {
            d1 += 1;
            d2 = 0;
            d3 = 1;
            d4 = if d1 == 2 { 3 } else { 2 };
            break;
        }

        if d2 == d3 {
            d3 += 1;
            d4 = 0;
        }
        if d2 == d4 {
            if d2 == 9 {
                d3 += 1;
                d4 = 0;
            } else {
                d4 += 1;
            }
        }

        if d3 == 9 && d4 == 9 {
            d2 += if d1 == d2 + 1 { 2 } else { 1 };
            d3 = 0;
            if d1 == 1 || d2 == 1 {
                if d1 == 2 || d2 == 2 {
                    d4 = 3;
                } else {
                    d4 = 2;
                }
            } else {
                d4 = 1;
            }
            break;
        }
        if d3 == d4 {
            d4 += 1;
        }
    }

    from_4_digits(d1, d2, d3, d4)
}
