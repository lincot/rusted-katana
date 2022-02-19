//! <https://www.codewars.com/kata/57873ab5e55533a2890000c7/train/rust>

pub fn time_correct(time_str: &str) -> Option<String> {
    let (hours, right) = time_str.split_once(':')?;
    let (minutes, seconds) = right.split_once(':')?;

    let mut hours: u8 = hours.parse().ok()?;
    let mut minutes: u8 = minutes.parse().ok()?;
    let mut seconds: u8 = seconds.parse().ok()?;

    minutes += seconds / 60;
    seconds %= 60;
    hours += minutes / 60;
    minutes %= 60;
    hours %= 24;

    Some(format!("{:02}:{:02}:{:02}", hours, minutes, seconds))
}
