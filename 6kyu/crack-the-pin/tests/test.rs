use crack_the_pin::crack;

#[test]
#[cfg(not(miri))]
fn test() {
    assert_eq!(crack("827ccb0eea8a706c4c34a16891f84e7b"), "12345");
    assert_eq!(crack("86aa400b65433b608a9db30070ec60cd"), "00078");
}
