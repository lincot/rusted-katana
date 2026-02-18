//! <https://www.codewars.com/kata/633a870b198a4c00286ad2b7/train/rust>

pub fn life_predictor(date: &str) -> String {
    let date = date.as_bytes();
    assert!(date.len() == 10);
    let year = 1000 * (date[0] - b'0') as u16
        + 100 * (date[1] - b'0') as u16
        + 10 * (date[2] - b'0') as u16
        + (date[3] - b'0') as u16;
    let month = 10 * (date[5] - b'0') + date[6] - b'0';
    let day = 10 * (date[8] - b'0') + date[9] - b'0';

    let (mut first, mut reset) = MONTHS[(month - 1) as usize];
    let is_leap = is_leap_year(year);
    if is_leap && ((3..=11).contains(&month) || month == 12 && day <= 5) {
        first += 1;
        reset -= 1;
    }

    let (target_month, target_day, decrement_year) = if day < reset {
        (
            if month < 11 { month + 2 } else { month - 10 },
            first + day,
            month <= 10,
        )
    } else {
        (
            if month < 10 { month + 3 } else { month - 9 },
            day - reset + 1,
            month < 10,
        )
    };

    let mut res = vec![b'-'; 10];
    res[..4].copy_from_slice(&date[..4]);
    if decrement_year {
        for digit in res[..4].iter_mut().rev() {
            if *digit != b'0' {
                *digit -= 1;
                break;
            }
            *digit = b'9';
        }
    }

    res[5] = b'0' + target_month / 10;
    res[6] = b'0' + target_month % 10;
    res[8] = b'0' + target_day / 10;
    res[9] = b'0' + target_day % 10;

    unsafe { String::from_utf8_unchecked(res) }
}

const fn is_leap_year(year: u16) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

const MONTHS: [(u8, u8); 12] = [
    (26, 6),
    (26, 5),
    (24, 8),
    (24, 7),
    (24, 8),
    (24, 8),
    (23, 8),
    (24, 8),
    (24, 7),
    (24, 8),
    (24, 8),
    (23, 6),
];
