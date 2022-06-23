//! <https://www.codewars.com/kata/577ff15ad648a14b780000e7/train/rust>

pub const fn greet<'a, 'b>(language: &'a str) -> &'b str {
    match language.as_bytes() {
        b"czech" => "Vitejte",
        b"danish" => "Velkomst",
        b"dutch" => "Welkom",
        b"estonian" => "Tere tulemast",
        b"finnish" => "Tervetuloa",
        b"flemish" => "Welgekomen",
        b"french" => "Bienvenue",
        b"german" => "Willkommen",
        b"irish" => "Failte",
        b"italian" => "Benvenuto",
        b"latvian" => "Gaidits",
        b"lithuanian" => "Laukiamas",
        b"polish" => "Witamy",
        b"spanish" => "Bienvenido",
        b"swedish" => "Valkommen",
        b"welsh" => "Croeso",
        _ => "Welcome",
    }
}
