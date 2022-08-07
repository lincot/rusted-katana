//! <https://www.codewars.com/kata/57e8f757085f7c7d6300009a/train/rust>

use my_prelude::prelude::*;

pub fn plane_seat(seat_number: &str) -> String {
    const NO_SEAT: &str = "No Seat!!";

    let seat_number = seat_number.as_bytes();

    let (number, letter) = match seat_number.len() {
        3 => (
            10 * (seat_number[0] - b'0') + (seat_number[1] - b'0'),
            seat_number[2],
        ),
        2 => (seat_number[0] - b'0', seat_number[1]),
        _ => return NO_SEAT.into(),
    };

    let mut res = String::with_capacity("Middle-Middle".len());

    unsafe {
        res.push_str_unchecked(match number {
            1..=20 => "Front-",
            21..=40 => "Middle-",
            41..=60 => "Back-",
            _ => return NO_SEAT.into(),
        });
    }

    unsafe {
        res.push_str_unchecked(match letter {
            b'A'..=b'C' => "Left",
            b'D'..=b'F' => "Middle",
            b'G' | b'H' | b'K' => "Right",
            _ => return NO_SEAT.into(),
        });
    }

    res
}
