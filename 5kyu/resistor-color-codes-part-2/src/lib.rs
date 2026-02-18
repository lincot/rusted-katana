//! <https://www.codewars.com/kata/5855777bb45c01bada0002ac/train/rust>

use unchecked_std::prelude::*;

pub fn encode_resistor_colors(s: &str) -> String {
    let s = s.as_bytes();
    let mut res = String::with_capacity("violet violet violet gold".len());

    let first_color = encode_color(s[0]);

    unsafe {
        res.push_str_unchecked(first_color);

        match s[1] {
            b'k' => res.push_str_unchecked(" black red"),
            b'M' => res.push_str_unchecked(" black green"),
            b'.' => {
                let second_color = encode_color(s[2]);
                res.push_unchecked(' ');
                res.push_str_unchecked(second_color);
                if s[3] == b'k' {
                    res.push_str_unchecked(" red");
                } else {
                    res.push_str_unchecked(" green");
                }
            }
            b => {
                let second_color = encode_color(b);
                res.push_unchecked(' ');
                res.push_str_unchecked(second_color);

                let mut n_zeros = s.len() - " ohms".len() - 2;
                match s[n_zeros + 1] {
                    b'k' => n_zeros += 2,
                    b'M' => n_zeros += 5,
                    _ => {}
                }
                let third_color = encode_color(b'0' + n_zeros as u8);
                res.push_unchecked(' ');
                res.push_str_unchecked(third_color);
            }
        }

        res.push_str_unchecked(" gold");
    }

    res
}

const fn encode_color(color: u8) -> &'static str {
    match color {
        b'0' => "black",
        b'1' => "brown",
        b'2' => "red",
        b'3' => "orange",
        b'4' => "yellow",
        b'5' => "green",
        b'6' => "blue",
        b'7' => "violet",
        b'8' => "gray",
        _ => "white",
    }
}
