//! <https://www.codewars.com/kata/6411b91a5e71b915d237332d/train/rust>

pub fn valid_parentheses(parens: &str) -> bool {
    let mut op = 0;
    for b in parens.bytes() {
        if b == b'(' {
            op += 1;
        } else if op == 0 {
            return false;
        } else {
            op -= 1;
        }
    }
    op == 0
}
