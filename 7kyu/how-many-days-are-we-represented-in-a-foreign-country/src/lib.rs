//! <https://www.codewars.com/kata/58e93b4706db4d24ee000096/train/rust>

pub fn days_represented(trips: &[(u32, u32)]) -> u32 {
    let mut days = [false; 365];

    for &(arrival, departure) in trips {
        assert!((1..=365).contains(&arrival));
        assert!((1..=365).contains(&departure));

        for day in (arrival - 1) as usize..departure as usize {
            let represented = unsafe { days.get_unchecked_mut(day) };
            *represented = true;
        }
    }

    days.into_iter().filter(|&x| x).count() as _
}
