//! <https://www.codewars.com/kata/57873ab5e55533a2890000c7/train/rust>

use unchecked::{ExtendFromSliceUnchecked, PushUnchecked};

const TABLE: [[u8; 2]; 60] = [
    *b"00", *b"01", *b"02", *b"03", *b"04", *b"05", *b"06", *b"07", *b"08", *b"09", *b"10", *b"11",
    *b"12", *b"13", *b"14", *b"15", *b"16", *b"17", *b"18", *b"19", *b"20", *b"21", *b"22", *b"23",
    *b"24", *b"25", *b"26", *b"27", *b"28", *b"29", *b"30", *b"31", *b"32", *b"33", *b"34", *b"35",
    *b"36", *b"37", *b"38", *b"39", *b"40", *b"41", *b"42", *b"43", *b"44", *b"45", *b"46", *b"47",
    *b"48", *b"49", *b"50", *b"51", *b"52", *b"53", *b"54", *b"55", *b"56", *b"57", *b"58", *b"59",
];

pub fn time_correct(time_str: &str) -> Option<String> {
    let (hours, rest) = time_str.split_once(':')?;
    let (minutes, seconds) = rest.split_once(':')?;

    let mut hours: u8 = hours.parse().ok()?;
    let mut minutes: u8 = minutes.parse().ok()?;
    let mut seconds: u8 = seconds.parse().ok()?;

    minutes += seconds / 60;
    seconds %= 60;
    hours += minutes / 60;
    minutes %= 60;
    hours %= 24;

    let mut res = Vec::with_capacity("00:00:00".len());
    unsafe {
        res.extend_from_slice_unchecked(&TABLE[hours as usize]);
        res.push_unchecked(b':');
        res.extend_from_slice_unchecked(&TABLE[minutes as usize]);
        res.push_unchecked(b':');
        res.extend_from_slice_unchecked(&TABLE[seconds as usize]);
        Some(String::from_utf8_unchecked(res))
    }
}
