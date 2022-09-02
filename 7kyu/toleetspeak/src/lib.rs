//! <https://www.codewars.com/kata/57c1ab3949324c321600013f/train/rust>

pub fn to_leet_speak(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    unsafe { res.as_mut_vec().set_len(s.len()) };
    let mut res_ptr = res.as_mut_ptr();
    for b in s.bytes() {
        unsafe {
            *res_ptr = match b {
                b'A' => b'@',
                b'B' => b'8',
                b'C' => b'(',
                b'E' => b'3',
                b'G' => b'6',
                b'H' => b'#',
                b'I' => b'!',
                b'L' => b'1',
                b'O' => b'0',
                b'S' => b'$',
                b'T' => b'7',
                b'Z' => b'2',
                b => b,
            };
            res_ptr = res_ptr.add(1);
        }
    }
    res
}
