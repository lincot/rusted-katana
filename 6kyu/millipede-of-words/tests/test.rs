use millipede_of_words::millipede;

#[test]
fn test() {
    assert!(!millipede(&["strike", "elephant", "thesis", "pain"]));
    assert!(millipede(&["cycle", "exotic", "cable"]));
    assert!(millipede(&["evaluate", "endorse", "engine"]));
}
