//! <https://www.codewars.com/kata/5cb5d96bed8828002a24aedb/train/rust>

#[macro_export]
macro_rules! add {
    ( $( $x:expr ),* ) => { 0 $( + $x )* };
}
