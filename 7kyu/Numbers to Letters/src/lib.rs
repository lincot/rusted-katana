//! <https://www.codewars.com/kata/57ebaa8f7b45ef590c00000c/train/rust>

pub fn switcher(numbers: Vec<&str>) -> String {
    let chars = b"zyxwvutsrqponmlkjihgfedcba!? ";

    unsafe {
        String::from_utf8_unchecked(
            numbers
                .into_iter()
                .map(|s| {
                    let s = s.as_bytes();

                    let n = match s.len() {
                        2 => (10 * (s[0] - b'0') + (s[1] - b'0')),
                        1 => s[0] - b'0',
                        _ => panic!(),
                    } - 1;

                    chars[n as usize]
                })
                .collect(),
        )
    }
}
