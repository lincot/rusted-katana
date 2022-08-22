//! <https://www.codewars.com/kata/591588d49f4056e13f000001/train/rust>

use my_prelude::prelude::*;

pub fn hq9(code: &str) -> Option<String> {
    match code {
        "H" => Some("Hello World!".into()),
        "Q" => Some("Q".into()),
        "9" => {
            let mut bytes = Vec::with_capacity(11785);

            unsafe {
                bytes.extend_from_slice_unchecked(
                    b"99 bottles of beer on the wall, 99 bottles of beer.\n",
                );
            }

            for i in (2u8..99u8).rev() {
                unsafe {
                    bytes.extend_from_slice_unchecked(b"Take one down and pass it around, ");
                    bytes.write_num_unchecked(i);
                    bytes.extend_from_slice_unchecked(b" bottles of beer on the wall.\n");
                    bytes.write_num_unchecked(i);
                    bytes.extend_from_slice_unchecked(b" bottles of beer on the wall, ");
                    bytes.write_num_unchecked(i);
                    bytes.extend_from_slice_unchecked(b" bottles of beer.\n");
                }
            }

            unsafe {
                bytes.extend_from_slice_unchecked(
                    b"Take one down and pass it around, 1 bottle of beer on the wall.
1 bottle of beer on the wall, 1 bottle of beer.
Take one down and pass it around, no more bottles of beer on the wall.
No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.",
                );
            }

            Some(unsafe { String::from_utf8_unchecked(bytes) })
        }
        _ => None,
    }
}