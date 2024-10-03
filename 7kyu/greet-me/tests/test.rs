use greet_me::greet;

#[test]
fn test() {
    assert_eq!(greet("alice"), "Hello Alice!");
    assert_eq!(greet("alİce"), "Hello Ali̇ce!");
    assert_eq!(greet("алиСА"), "Hello Алиса!");
}
