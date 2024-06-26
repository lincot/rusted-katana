//! <https://www.codewars.com/kata/56bcaedfcf6b7f2125001118/train/rust>

use unchecked_std::prelude::*;

pub fn html_special_chars(html: &str) -> String {
    let cap = "&quot;".len() * html.len();
    let mut res = String::with_capacity(cap);
    for b in html.bytes() {
        unsafe {
            match b {
                b'&' => res.push_str_unchecked("&amp;"),
                b'<' => res.push_str_unchecked("&lt;"),
                b'>' => res.push_str_unchecked("&gt;"),
                b'"' => res.push_str_unchecked("&quot;"),
                _ => res.as_mut_vec().push_unchecked(b),
            }
        }
    }
    res
}
