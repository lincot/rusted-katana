//! <https://www.codewars.com/kata/568dc014440f03b13900001d/train/rust>

pub fn get_drink_by_profession(param: &str) -> &'static str {
    match param.to_ascii_lowercase().as_bytes() {
        b"jabroni" => "Patron Tequila",
        b"school counselor" => "Anything with Alcohol",
        b"programmer" => "Hipster Craft Beer",
        b"bike gang member" => "Moonshine",
        b"politician" => "Your tax dollars",
        b"rapper" => "Cristal",
        _ => "Beer",
    }
}
