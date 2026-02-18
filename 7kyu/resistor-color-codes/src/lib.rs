//! <https://www.codewars.com/kata/57cf3dad05c186ba22000348/train/rust>

use unchecked_std::prelude::*;

pub fn decode_resistor_colors(bands: &str) -> String {
    let mut bands = bands.as_bytes();
    let mut res = Vec::with_capacity("23000M ohms, 20%".len());
    let (first_digit, len) = decode_color(bands);
    bands = &bands[len + 1..];
    let (second_digit, len) = decode_color(bands);
    bands = &bands[len + 1..];
    let (mut n_zeros, len) = decode_color(bands);

    unsafe {
        res.push_unchecked(b'0' + first_digit);

        if second_digit == 0 {
            n_zeros += 1;
        } else {
            if n_zeros == 2 || n_zeros == 5 {
                res.push_unchecked(b'.');
            }
            res.push_unchecked(b'0' + second_digit);
        }

        let magn = if n_zeros >= 6 || n_zeros == 5 && second_digit != 0 {
            n_zeros = n_zeros.saturating_sub(6);
            Some(b'M')
        } else if n_zeros >= 3 || n_zeros == 2 && second_digit != 0 {
            n_zeros = n_zeros.saturating_sub(3);
            Some(b'k')
        } else {
            None
        };
        res.push_many_unchecked(b'0', n_zeros as usize);
        if let Some(magn) = magn {
            res.push_unchecked(magn);
        }

        res.extend_from_slice_unchecked(b" ohms, ");

        match bands.get(len + 1) {
            Some(&b'g') => res.extend_from_slice_unchecked(b"5%"),
            Some(_) => res.extend_from_slice_unchecked(b"10%"),
            None => res.extend_from_slice_unchecked(b"20%"),
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}

fn decode_color(s: &[u8]) -> (u8, usize) {
    match s[0] {
        b'b' => match s[2] {
            b'a' => (0, "black".len()),
            b'o' => (1, "brown".len()),
            _ => (6, "blue".len()),
        },
        b'r' => (2, "red".len()),
        b'o' => (3, "orange".len()),
        b'y' => (4, "yellow".len()),
        b'v' => (7, "violet".len()),
        b'w' => (9, "white".len()),
        _ => {
            if s[2] == b'e' {
                (5, "green".len())
            } else {
                (8, "gray".len())
            }
        }
    }
}
