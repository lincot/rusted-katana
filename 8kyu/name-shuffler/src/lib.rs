//! <https://www.codewars.com/kata/559ac78160f0be07c200005a/train/rust>

pub fn name_shuffler(s: &str) -> String {
    let mut res = Vec::with_capacity(s.len());

    let mut bytes = s.bytes();
    loop {
        match bytes.next() {
            Some(b) => {
                if b == b' ' {
                    break;
                }
            }
            None => panic!(),
        }
    }
    res.extend(bytes);

    res.push(b' ');

    res.extend(s.bytes().take_while(|&c| c != b' '));

    unsafe { String::from_utf8_unchecked(res) }
}
