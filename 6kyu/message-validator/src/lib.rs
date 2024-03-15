//! <https://www.codewars.com/kata/5fc7d2d2682ff3000e1a3fbc/train/rust>

pub fn is_valid_message(msg: &str) -> bool {
    let mut chunk_start = 0;
    while chunk_start < msg.len() {
        let Some(substring_start) = msg.as_bytes()[chunk_start..]
            .iter()
            .position(|&b| b > b'9')
            .map(|x| x + chunk_start)
        else {
            return false;
        };

        let next_chunk_start = unsafe { msg.as_bytes().get_unchecked(substring_start..) }
            .iter()
            .position(|&b| b <= b'9')
            .map_or(msg.len(), |x| x + substring_start);
        let Ok(number) =
            unsafe { msg.get_unchecked(chunk_start..substring_start) }.parse::<usize>()
        else {
            return false;
        };

        if number != next_chunk_start - substring_start {
            return false;
        }

        chunk_start = next_chunk_start;
    }

    true
}
