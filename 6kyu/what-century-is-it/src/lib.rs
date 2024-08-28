//! <https://www.codewars.com/kata/52fb87703c1351ebd200081f/train/rust>

pub fn what_century(year: &str) -> String {
    if year.starts_with("99") {
        return "100th".into();
    }

    let year = year.as_bytes();
    assert!(year.len() == 4 && year[0].is_ascii_digit() && year[1].is_ascii_digit());

    let mut res = Vec::with_capacity(4);
    let spare = res.spare_capacity_mut();

    if &year[2..=3] == b"00" {
        spare[0].write(year[0]);
        spare[1].write(year[1]);
    } else if year[1] == b'9' {
        spare[0].write(year[0] + 1);
        spare[1].write(b'0');
    } else {
        spare[0].write(year[0]);
        spare[1].write(year[1] + 1);
    }

    unsafe {
        if spare[0].assume_init_read() == b'1' {
            spare[2].write(b't');
            spare[3].write(b'h');
        } else if spare[1].assume_init_read() == b'1' {
            spare[2].write(b's');
            spare[3].write(b't');
        } else if spare[1].assume_init_read() == b'2' {
            spare[2].write(b'n');
            spare[3].write(b'd');
        } else if spare[1].assume_init_read() == b'3' {
            spare[2].write(b'r');
            spare[3].write(b'd');
        } else {
            spare[2].write(b't');
            spare[3].write(b'h');
        }
    }

    unsafe {
        res.set_len(4);
        String::from_utf8_unchecked(res)
    }
}
