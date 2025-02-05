#[test]
fn test_name_string() {
    let s: String = String::from("krishnaik06 /");
    let t = s.split('/').collect::<Vec<_>>();
    let mut res = t
        .iter()
        .map(|&x| x.to_string().trim().to_string())
        .collect::<String>();
    res.push('/');
    assert_eq!(String::from("krishnaik06/"), res);
}
