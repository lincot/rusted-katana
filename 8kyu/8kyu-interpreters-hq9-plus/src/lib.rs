//! <https://www.codewars.com/kata/591588d49f4056e13f000001/train/rust>

pub fn hq9(code: &str) -> Option<String> {
    match code.as_bytes() {
        b"H" => Some("Hello World!".into()),
        b"Q" => Some("Q".into()),
        b"9" => {
            let mut bytes = Vec::with_capacity(11785);

            bytes.extend(b"99 bottles of beer on the wall, 99 bottles of beer.\n");

            for i in (2u8..99u8).rev() {
                let i = i.to_string();
                bytes.extend(b"Take one down and pass it around, ");
                bytes.extend(i.bytes());
                bytes.extend(b" bottles of beer on the wall.\n");
                bytes.extend(i.bytes());
                bytes.extend(b" bottles of beer on the wall, ");
                bytes.extend(i.bytes());
                bytes.extend(b" bottles of beer.\n");
            }

            bytes.extend(
                b"Take one down and pass it around, 1 bottle of beer on the wall.
1 bottle of beer on the wall, 1 bottle of beer.
Take one down and pass it around, no more bottles of beer on the wall.
No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.",
            );

            Some(unsafe { String::from_utf8_unchecked(bytes) })
        }
        _ => None,
    }
}
