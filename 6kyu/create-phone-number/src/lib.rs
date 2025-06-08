//! <https://www.codewars.com/kata/525f50e3b73515a6db000b83/train/rust>

pub fn create_phone_number(numbers: &[u8]) -> String {
    assert!(numbers.len() == 10);
    unsafe {
        String::from_utf8_unchecked(vec![
            b'(',
            b'0' + numbers[0],
            b'0' + numbers[1],
            b'0' + numbers[2],
            b')',
            b' ',
            b'0' + numbers[3],
            b'0' + numbers[4],
            b'0' + numbers[5],
            b'-',
            b'0' + numbers[6],
            b'0' + numbers[7],
            b'0' + numbers[8],
            b'0' + numbers[9],
        ])
    }
}
