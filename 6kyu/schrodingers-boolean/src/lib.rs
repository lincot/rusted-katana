//! <https://www.codewars.com/kata/5a5f9f80f5dc3f942b002309/train/rust>

#[derive(Debug)]
pub struct Omnibool;

impl PartialEq<bool> for Omnibool {
    fn eq(&self, _: &bool) -> bool {
        true
    }
}

#[allow(non_upper_case_globals)]
pub const omnibool: Omnibool = Omnibool;
