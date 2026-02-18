use name_array_capping::cap_me;

#[test]
fn test() {
    assert_eq!(cap_me(vec!["alice".into()]), ["Alice"]);
    assert_eq!(cap_me(vec!["alİce".into()]), ["Ali̇ce"]);
    assert_eq!(cap_me(vec!["алиСА".into()]), ["Алиса"]);
    assert_eq!(cap_me(vec!["Jarosław".into()]), ["Jarosław"]);
}
