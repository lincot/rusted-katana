pub trait DigitFormat {
    const REVERSED: bool;
    const RAW: bool;
}

pub struct Normal;
pub struct Reversed;
pub struct Raw;
pub struct ReversedRaw;

impl DigitFormat for Normal {
    const REVERSED: bool = false;
    const RAW: bool = false;
}

impl DigitFormat for Raw {
    const REVERSED: bool = false;
    const RAW: bool = true;
}

impl DigitFormat for Reversed {
    const REVERSED: bool = true;
    const RAW: bool = false;
}

impl DigitFormat for ReversedRaw {
    const REVERSED: bool = true;
    const RAW: bool = true;
}

pub trait Next2DigitsFormat {
    const RAW: bool;
}

impl Next2DigitsFormat for Normal {
    const RAW: bool = false;
}

impl Next2DigitsFormat for Raw {
    const RAW: bool = true;
}

#[derive(PartialEq, Eq, Debug)]
pub enum Base {
    Base2,
    Base10,
    Base16,
}

impl From<Base> for u8 {
    fn from(value: Base) -> Self {
        match value {
            Base::Base2 => 2,
            Base::Base10 => 10,
            Base::Base16 => 16,
        }
    }
}

pub trait Radix {
    const BASE: Base;
}

pub struct Base2;
pub struct Base10;
pub struct Base16;

impl Radix for Base2 {
    const BASE: Base = Base::Base2;
}

impl Radix for Base10 {
    const BASE: Base = Base::Base10;
}

impl Radix for Base16 {
    const BASE: Base = Base::Base16;
}
