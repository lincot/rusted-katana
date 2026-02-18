//! <https://www.codewars.com/kata/687dd1e199ab3955339b8071/train/rust>

use core::str::FromStr;

pub fn can_play(hand: &[&str], face_up: &str) -> bool {
    let face_up: Card = face_up.parse().unwrap();
    for card in hand {
        let card: Card = card.parse().unwrap();
        if card.color == face_up.color || card.number == face_up.number {
            return true;
        }
    }
    false
}

struct Card {
    color: u8,
    number: u8,
}

impl FromStr for Card {
    type Err = ();

    fn from_str(card: &str) -> Result<Self, Self::Err> {
        let card = card.as_bytes();
        Ok(Self {
            color: *card.first().ok_or(())?,
            number: *card.last().ok_or(())?,
        })
    }
}
