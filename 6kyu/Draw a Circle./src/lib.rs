//! <https://www.codewars.com/kata/59c804d923dacc6c41000004/train/rust>

pub fn circle(radius: i32) -> String {
    if radius < 0 {
        return String::new();
    }
    if radius == 0 {
        return "\n".into();
    }
    let radius = radius as usize;

    let cap = "█".len() * (2 * radius - 1).pow(2) + "\n".len() * (2 * radius - 1);
    let mut res = String::with_capacity(cap);

    (0..2 * radius - 1).for_each(|row| {
        let dist_to_center = row.max(radius - 1) - row.min(radius - 1);
        let half_width =
            (((radius.pow(2) - dist_to_center.pow(2)) as f64).sqrt() + 0.999999) as usize;

        res.push_str(&" ".repeat(radius - half_width));
        res.push_str(&"█".repeat(2 * half_width - 1));
        res.push_str(&" ".repeat(radius - half_width));
        res.push('\n');
    });

    res
}
