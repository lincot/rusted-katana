//! <https://www.codewars.com/kata/591588d49f4056e13f000001/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use digital::WriteNumUnchecked;
use unchecked::PushStrUnchecked;

pub fn hq9(code: &str) -> Option<String> {
    match code {
        "H" => Some("Hello World!".into()),
        "Q" => Some("Q".into()),
        "9" => {
            let mut res = String::with_capacity(11785);

            unsafe {
                res.push_str_unchecked("99 bottles of beer on the wall, 99 bottles of beer.\n");
            }

            for i in (2u8..99).rev() {
                unsafe {
                    res.push_str_unchecked("Take one down and pass it around, ");
                    res.write_num_unchecked(i, 10, false, false);
                    res.push_str_unchecked(" bottles of beer on the wall.\n");
                    res.write_num_unchecked(i, 10, false, false);
                    res.push_str_unchecked(" bottles of beer on the wall, ");
                    res.write_num_unchecked(i, 10, false, false);
                    res.push_str_unchecked(" bottles of beer.\n");
                }
            }

            unsafe {
                res.push_str_unchecked(
                    "Take one down and pass it around, 1 bottle of beer on the wall.
1 bottle of beer on the wall, 1 bottle of beer.
Take one down and pass it around, no more bottles of beer on the wall.
No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.",
                );
            }

            Some(res)
        }
        _ => None,
    }
}
