//! <https://www.codewars.com/kata/587c0138110b20624e000253/train/rust>

pub fn interpreter(tape: &str, data: &str) -> String {
    let mut res = data.to_string();
    if res.is_empty() {
        return res;
    }
    let data = unsafe { res.as_bytes_mut() };

    let mut i = 0;
    loop {
        for command in tape.bytes() {
            match command {
                b'0' => {
                    i += 1;
                    if i >= data.len() {
                        return res;
                    }
                }
                b'1' => {
                    if i >= data.len() {
                        unsafe { core::hint::unreachable_unchecked() };
                    }
                    data[i] = match data[i] {
                        b'0' => b'1',
                        b'1' => b'0',
                        _ => panic!(),
                    }
                }
                _ => panic!(),
            }
        }
    }
}
