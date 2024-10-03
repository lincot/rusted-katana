use fix_string_case::solve;

#[test]
fn test() {
    assert_eq!(solve("CODe"), "CODE");
    assert_eq!(solve("КОДе"), "КОДЕ");
    assert_eq!(solve("КОДе"), "КОДЕ");
    assert_eq!(solve("İode"), "i̇ode");
}
