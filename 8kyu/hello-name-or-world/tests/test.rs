use hello_name_or_world::hello;

#[test]
fn test() {
    assert_eq!(hello("aliCE"), "Hello, Alice!");
    assert_eq!(hello("алİСА"), "Hello, Алi̇са!");
}
