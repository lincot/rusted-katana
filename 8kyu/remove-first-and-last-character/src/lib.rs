//! <https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust>

pub fn remove_char(s: &str) -> String {
    unsafe {
        s.get_unchecked(
            s.chars().next().unwrap().len_utf8()
                ..s.len() - s.chars().next_back().unwrap().len_utf8(),
        )
    }
    .into()
}
