use unique_strings::uniq_count;

#[test]
fn test() {
    assert_eq!(uniq_count("ABC"), 6u8.into());
    assert_eq!(uniq_count("AİC"), 6u8.into());
    assert_eq!(uniq_count("İİİ"), 1u8.into());
    assert_eq!(uniq_count("АбцД"), 24u8.into());
}
