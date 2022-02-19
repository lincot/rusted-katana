//! <https://www.codewars.com/kata/56bcaedfcf6b7f2125001118/train/rust>

pub fn html_special_chars(html: &str) -> String {
    // arbitrary capacity
    let cap = html.len() + html.len() / 3;
    let mut res = Vec::with_capacity(cap);

    html.bytes().for_each(|b| match b {
        b'&' => res.extend(b"&amp;"),
        b'<' => res.extend(b"&lt;"),
        b'>' => res.extend(b"&gt;"),
        b'"' => res.extend(b"&quot;"),
        _ => res.push(b),
    });

    unsafe { String::from_utf8_unchecked(res) }
}
