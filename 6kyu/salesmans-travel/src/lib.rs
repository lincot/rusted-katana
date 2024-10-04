//! <https://www.codewars.com/kata/56af1a20509ce5b9b000001e/train/rust>

use unchecked_std::prelude::*;

pub fn travel(r: &str, zipcode: &str) -> String {
    let mut res = String::with_capacity(r.len() + zipcode.len() + 2);
    unsafe {
        res.push_str_unchecked(zipcode);
        res.push_unchecked(':');
        if zipcode.len() != 2 + 1 + 5 {
            res.push_unchecked('/');
            return res;
        }
    }

    let mut house_numbers = Vec::with_capacity(r.len() / "1 a OH 56432".len());

    // str::split makes it faster than <[u8]>::split somehow
    for address in r.split(',') {
        if address.ends_with(zipcode)
            && address.as_bytes().get(address.len() - zipcode.len() - 1) == Some(&b' ')
        {
            let number_start = address
                .as_bytes()
                .iter()
                .position(|&b| b.is_ascii_digit())
                .unwrap();
            unsafe {
                let number_end = number_start
                    + address
                        .as_bytes()
                        .get_unchecked(number_start..)
                        .iter()
                        .position(|&b| b == b' ')
                        .unwrap();
                house_numbers.push_unchecked(address.get_unchecked(number_start..number_end));
                res.push_str_unchecked(
                    address.get_unchecked(number_end + 1..address.len() - zipcode.len() - 1),
                );
                res.push_unchecked(',');
            }
        }
    }

    unsafe {
        if let Some(house_number) = house_numbers.first() {
            *res.as_mut_vec().last_mut().unwrap_unchecked() = b'/';
            res.push_str_unchecked(house_number);
            for house_number in &house_numbers[1..] {
                res.push_unchecked(',');
                res.push_str_unchecked(house_number);
            }
        } else {
            res.push_unchecked('/');
        }
    }

    res
}
