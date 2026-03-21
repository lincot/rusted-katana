use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ALPHA: HashMap<char, [&'static str; 7]> = HashMap::from_iter([
        (
            ' ',
            [
                "     ", "     ", "     ", "     ", "     ", "     ", "     "
            ]
        ),
        (
            'a',
            [
                " AAA ", "A   A", "A   A", "AAAAA", "A   A", "A   A", "A   A"
            ]
        ),
        (
            'b',
            [
                "BBBB ", "B   B", "B   B", "BBBB ", "B   B", "B   B", "BBBB "
            ]
        ),
        (
            'c',
            [
                " CCC ", "C   C", "C    ", "C    ", "C    ", "C   C", " CCC "
            ]
        ),
        (
            'd',
            [
                "DDDD ", "D   D", "D   D", "D   D", "D   D", "D   D", "DDDD "
            ]
        ),
        (
            'e',
            [
                "EEEEE", "E    ", "E    ", "EEEEE", "E    ", "E    ", "EEEEE"
            ]
        ),
        (
            'f',
            [
                "FFFFF", "F    ", "F    ", "FFFFF", "F    ", "F    ", "F    "
            ]
        ),
        (
            'g',
            [
                " GGG ", "G   G", "G    ", "G GGG", "G   G", "G   G", " GGG "
            ]
        ),
        (
            'h',
            [
                "H   H", "H   H", "H   H", "HHHHH", "H   H", "H   H", "H   H"
            ]
        ),
        (
            'i',
            [
                "IIIII", "  I  ", "  I  ", "  I  ", "  I  ", "  I  ", "IIIII"
            ]
        ),
        (
            'j',
            [
                "JJJJJ", "    J", "    J", "    J", "    J", "    J", "JJJJ "
            ]
        ),
        (
            'k',
            [
                "K   K", "K  K ", "K K  ", "KK   ", "K K  ", "K  K ", "K   K"
            ]
        ),
        (
            'l',
            [
                "L    ", "L    ", "L    ", "L    ", "L    ", "L    ", "LLLLL"
            ]
        ),
        (
            'm',
            [
                "M   M", "MM MM", "M M M", "M   M", "M   M", "M   M", "M   M"
            ]
        ),
        (
            'n',
            [
                "N   N", "NN  N", "N   N", "N N N", "N   N", "N  NN", "N   N"
            ]
        ),
        (
            'o',
            [
                " OOO ", "O   O", "O   O", "O   O", "O   O", "O   O", " OOO "
            ]
        ),
        (
            'p',
            [
                "PPPP ", "P   P", "P   P", "PPPP ", "P    ", "P    ", "P    "
            ]
        ),
        (
            'q',
            [
                " QQQ ", "Q   Q", "Q   Q", "Q   Q", "Q Q Q", "Q  QQ", " QQQQ"
            ]
        ),
        (
            'r',
            [
                "RRRR ", "R   R", "R   R", "RRRR ", "R R  ", "R  R ", "R   R"
            ]
        ),
        (
            's',
            [
                " SSS ", "S   S", "S    ", " SSS ", "    S", "S   S", " SSS "
            ]
        ),
        (
            't',
            [
                "TTTTT", "  T  ", "  T  ", "  T  ", "  T  ", "  T  ", "  T  "
            ]
        ),
        (
            'u',
            [
                "U   U", "U   U", "U   U", "U   U", "U   U", "U   U", " UUU "
            ]
        ),
        (
            'v',
            [
                "V   V", "V   V", "V   V", "V   V", "V   V", " V V ", "  V  "
            ]
        ),
        (
            'w',
            [
                "W   W", "W   W", "W   W", "W W W", "W W W", "W W W", " W W "
            ]
        ),
        (
            'x',
            [
                "X   X", "X   X", " X X ", "  X  ", " X X ", "X   X", "X   X"
            ]
        ),
        (
            'y',
            [
                "Y   Y", "Y   Y", " Y Y ", "  Y  ", "  Y  ", "  Y  ", "  Y  "
            ]
        ),
        (
            'z',
            [
                "ZZZZZ", "    Z", "   Z ", "  Z  ", " Z   ", "Z    ", "ZZZZZ"
            ]
        ),
    ]);
}

pub fn unbleach(s: &str) -> String {
    let res = s.replace(" ", "\u{2022}");
    let res = res.replace("\n", "\\n\n");
    res
}
