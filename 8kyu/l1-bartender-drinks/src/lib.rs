//! <https://www.codewars.com/kata/568dc014440f03b13900001d/train/rust>

pub fn get_drink_by_profession(param: &str) -> &'static str {
    for (profession, drink) in DRINKS {
        if param.len() == profession.len()
            && param
                .bytes()
                .zip(profession.bytes())
                .all(|(a, b)| a == b || a == b - (b'a' - b'A'))
        {
            return drink;
        }
    }

    "Beer"
}

const DRINKS: &[(&str, &str)] = &[
    ("jabroni", "Patron Tequila"),
    ("school counselor", "Anything with Alcohol"),
    ("programmer", "Hipster Craft Beer"),
    ("bike gang member", "Moonshine"),
    ("politician", "Your tax dollars"),
    ("rapper", "Cristal"),
];
