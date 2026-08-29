use mexican_wave::wave;

#[test]
fn test() {
    assert_eq!(
        wave("хелло "),
        ["Хелло ", "хЕлло ", "хеЛло ", "хелЛо ", "хеллО "]
    );
    assert_eq!(wave("ΐ"), ["Ι\u{308}\u{301}"]);
}
