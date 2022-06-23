//! <https://www.codewars.com/kata/58708934a44cfccca60000c4/train/rust>

pub fn highlight(code: &str) -> String {
    fn push_beginning(s: &mut String, token: u8) {
        match token {
            b'F' => s.push_str(r#"<span style="color: pink">"#),
            b'L' => s.push_str(r#"<span style="color: red">"#),
            b'R' => s.push_str(r#"<span style="color: green">"#),
            b'0'..=b'9' => s.push_str(r#"<span style="color: orange">"#),
            _ => {}
        }
    }

    fn push_end(s: &mut String, token: u8) {
        match token {
            b'(' | b')' => {}
            _ => s.push_str("</span>"),
        }
    }

    let cap = (1 + r#"<span style="color: orange"></span>"#.len()) * code.len();
    let mut res = String::with_capacity(cap);

    let mut code = code.bytes();

    let mut prev_token = if let Some(first) = code.next() {
        first
    } else {
        return res;
    };

    push_beginning(&mut res, prev_token);
    for b in code {
        unsafe { res.as_mut_vec() }.push(prev_token);

        if !(b == prev_token || (b'0'..=b'9').contains(&b) && (b'0'..=b'9').contains(&prev_token)) {
            push_end(&mut res, prev_token);
            push_beginning(&mut res, b);
        }

        prev_token = b;
    }
    unsafe { res.as_mut_vec() }.push(prev_token);
    push_end(&mut res, prev_token);

    res
}
