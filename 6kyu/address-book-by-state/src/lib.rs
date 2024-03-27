//! <https://www.codewars.com/kata/59d0ee709f0cbcf65400003b/train/rust>

use core::array;
use unchecked_core::{ExtendFromSliceUnchecked, PushUnchecked};

pub fn by_state(str: &str) -> String {
    let [mut arizona, mut california, mut idaho, mut indiana, mut massachusetts, mut oklahoma, mut pennsylvania, mut virginia] =
        array::from_fn(|_| vec![]);
    for line in str.split_terminator('\n') {
        let line = line.as_bytes();
        if line.len() < 2 {
            continue;
        }
        match &line[line.len() - 2..] {
            b"AZ" => arizona.push(&line[..line.len() - 2]),
            b"CA" => california.push(&line[..line.len() - 2]),
            b"ID" => idaho.push(&line[..line.len() - 2]),
            b"IN" => indiana.push(&line[..line.len() - 2]),
            b"MA" => massachusetts.push(&line[..line.len() - 2]),
            b"OK" => oklahoma.push(&line[..line.len() - 2]),
            b"PA" => pennsylvania.push(&line[..line.len() - 2]),
            b"VA" => virginia.push(&line[..line.len() - 2]),
            _ => {}
        }
    }

    let mut cap = str.len();
    for (state_len, friends_len) in [
        ("Arizona".len(), arizona.len()),
        ("California".len(), california.len()),
        ("Idaho".len(), idaho.len()),
        ("Indiana".len(), indiana.len()),
        ("Massachusetts".len(), massachusetts.len()),
        ("Oklahoma".len(), oklahoma.len()),
        ("Pennsylvania".len(), pennsylvania.len()),
        ("Virginia".len(), virginia.len()),
    ] {
        if friends_len != 0 {
            cap += (state_len - 2 + "\n..... ".len()) * friends_len + state_len + 2;
        }
    }
    let mut res = Vec::with_capacity(cap);

    for (state, mut friends) in [
        (b"Arizona" as &[u8], arizona),
        (b"California", california),
        (b"Idaho", idaho),
        (b"Indiana", indiana),
        (b"Massachusetts", massachusetts),
        (b"Oklahoma", oklahoma),
        (b"Pennsylvania", pennsylvania),
        (b"Virginia", virginia),
    ] {
        if friends.is_empty() {
            continue;
        }
        friends.sort_unstable();
        unsafe {
            if !res.is_empty() {
                res.extend_from_slice_unchecked(b"\n ");
            }
            res.extend_from_slice_unchecked(state);
            for friend in friends {
                res.extend_from_slice_unchecked(b"\n..... ");
                for &b in friend {
                    if b != b',' {
                        res.push_unchecked(b);
                    }
                }
                res.extend_from_slice_unchecked(state);
            }
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
