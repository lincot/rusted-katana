//! <https://www.codewars.com/kata/57cfd92c05c1864df2001563/train/rust>

pub fn vowel_back(s: &str) -> String {
    const LETTERS_MAP: [u8; 26] = {
        let mut map = [0; 26];
        let mut l = b'a';
        let mut i = 0;
        while i < map.len() {
            map[i] = match l {
                b'a' | b'i' | b'u' => l - 5,
                b'c' | b'o' => l - 1,
                b'd' => l - 3,
                b'e' => l - 4,
                _ => l + 9,
            };
            if map[i] > b'z' {
                map[i] -= b'z' - b'a' + 1;
            } else if map[i] < b'a' {
                map[i] += b'z' - b'a' + 1;
            }
            if matches!(map[i], b'c' | b'o' | b'd' | b'e') {
                map[i] = l;
            }

            l += 1;
            i += 1;
        }
        map
    };

    let bytes = s
        .as_bytes()
        .iter()
        .map(|&b| {
            if b.is_ascii_lowercase() {
                LETTERS_MAP[(b - b'a') as usize]
            } else {
                b
            }
        })
        .collect();
    unsafe { String::from_utf8_unchecked(bytes) }
}
