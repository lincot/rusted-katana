//! <https://www.codewars.com/kata/56bcaedfcf6b7f2125001118/train/rust>

use my_prelude::prelude::*;

pub fn html_special_chars(html: &str) -> String {
    let cap = "&quot;".len() * html.len();
    let mut res = Vec::with_capacity(cap);

    for b in html.bytes() {
        unsafe {
            match b {
                b'&' => res.extend_from_slice_unchecked(b"&amp;"),
                b'<' => res.extend_from_slice_unchecked(b"&lt;"),
                b'>' => res.extend_from_slice_unchecked(b"&gt;"),
                b'"' => res.extend_from_slice_unchecked(b"&quot;"),
                _ => res.push_unchecked(b),
            }
        }
    }

    unsafe { String::from_utf8_unchecked(res) }
}
