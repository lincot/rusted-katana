//! <https://www.codewars.com/kata/58708934a44cfccca60000c4/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub fn highlight(code: &str) -> String {
    unsafe fn push_unchecked_beginning(s: &mut String, token: u8) {
        match token {
            b'F' => s.push_str_unchecked(r#"<span style="color: pink">"#),
            b'L' => s.push_str_unchecked(r#"<span style="color: red">"#),
            b'R' => s.push_str_unchecked(r#"<span style="color: green">"#),
            token if token.is_ascii_digit() => {
                s.push_str_unchecked(r#"<span style="color: orange">"#);
            }
            _ => {}
        }
    }

    unsafe fn push_unchecked_end(s: &mut String, token: u8) {
        if !b"()".contains(&token) {
            s.push_str_unchecked("</span>");
        }
    }

    let cap = (1 + r#"<span style="color: orange"></span>"#.len()) * code.len();
    let mut res = String::with_capacity(cap);

    let mut code = code.bytes();

    let Some(mut prev_token) = code.next() else {
         return res;
    };

    unsafe { push_unchecked_beginning(&mut res, prev_token) };
    for b in code {
        unsafe { res.as_mut_vec().push_unchecked(prev_token) };

        if !(b == prev_token || b.is_ascii_digit() && prev_token.is_ascii_digit()) {
            unsafe {
                push_unchecked_end(&mut res, prev_token);
                push_unchecked_beginning(&mut res, b);
            }
        }

        prev_token = b;
    }
    unsafe {
        res.as_mut_vec().push_unchecked(prev_token);
        push_unchecked_end(&mut res, prev_token);
    }

    res
}
