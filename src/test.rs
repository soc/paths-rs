#[cfg(test)] extern crate serde_test;
#[cfg(test)] use self::serde_test::{Token, assert_tokens};

#[test]
fn absolute_path_from_00_ok() {
    assert!(::AbsolutePath::from("foo/bar/qux").is_ok(), r#"::AbsolutePath::from("foo/bar/qux")"#);
}
#[test]
fn absolute_path_from_01_ok() {
    assert!(::AbsolutePath::from("/foo/bar/qux").is_ok(), r#"::AbsolutePath::from("/foo/bar/qux")"#);
}
#[test]
fn absolute_path_from_02_ok() {
    assert!(::AbsolutePath::from("/foo/bar/qux/").is_ok(), r#"::AbsolutePath::from("/foo/bar/qux/")"#);
}
#[test]
fn absolute_path_from_03_ok() {
    assert!(::AbsolutePath::from("/foo//bar///qux////").is_ok(), r#"::AbsolutePath::from("/foo//bar///qux////")"#);
}

#[test]
fn absolute_path_from_01() {
    assert!(::AbsolutePath::from("/foo\\b:ar\\aS?s").is_err(), r#"::AbsolutePath::from("/foo\\b:ar\\aS?s")"#);
}
#[test]
fn absolute_path_from_02() {
    assert!(::AbsolutePath::from("/foo\\b").is_err(), r#"::AbsolutePath::from("/foo\\b")"#);
}
#[test]
fn absolute_path_from_03() {
    assert!(::AbsolutePath::from("/fooS?s").is_err(), r#"::AbsolutePath::from("/fooS?s")"#);
}
#[test]
fn absolute_path_from_04() {
    assert!(::AbsolutePath::from("/foo%").is_err(), r#"::AbsolutePath::from("/foo%")"#);
}
#[test]
fn absolute_path_from_05() {
    assert!(::AbsolutePath::from("/foo*").is_err(), r#"::AbsolutePath::from("/foo*")"#);
}
#[test]
fn absolute_path_from_06() {
    assert!(::AbsolutePath::from("/foo/b:ar").is_err(), r#"::AbsolutePath::from("/foo/b:ar")"#);
}
#[test]
fn absolute_path_from_07() {
    assert!(::AbsolutePath::from("/foo\"bs").is_err(), r#"::AbsolutePath::from("/foo\"bs")"#);
}
#[test]
fn absolute_path_from_08() {
    assert!(::AbsolutePath::from("/foo<").is_err(), r#"::AbsolutePath::from("/foo<")"#);
}
#[test]
fn absolute_path_from_09() {
    assert!(::AbsolutePath::from("/foo>").is_err(), r#"::AbsolutePath::from("/foo>")"#);
}

#[test]
fn relative_path_from_00_ok() {
    assert!(::RelativePath::from("foo/bar/qux").is_ok(), r#"::RelativePath::from("foo/bar/qux")"#);
}
#[test]
fn relative_path_from_01_ok() {
    assert!(::RelativePath::from("/foo/bar/qux").is_ok(), r#"::RelativePath::from("/foo/bar/qux")"#);
}
#[test]
fn relative_path_from_02_ok() {
    assert!(::RelativePath::from("/foo/bar/qux/").is_ok(), r#"::RelativePath::from("/foo/bar/qux/")"#);
}
#[test]
fn relative_path_from_03_ok() {
    assert!(::RelativePath::from("/foo//bar///qux////").is_ok(), r#"::RelativePath::from("/foo//bar///qux////")"#);
}
#[test]
fn relative_path_from_01() {
    assert!(::RelativePath::from("/foo\\b:ar\\aS?s").is_err(), r#"::RelativePath::from("/foo\\b:ar\\aS?s")"#);
}
#[test]
fn relative_path_from_02() {
    assert!(::RelativePath::from("/foo\\b").is_err(), r#"::RelativePath::from("/foo\\b")"#);
}
#[test]
fn relative_path_from_03() {
    assert!(::RelativePath::from("/fooS?s").is_err(), r#"::RelativePath::from("/fooS?s")"#);
}
#[test]
fn relative_path_from_04() {
    assert!(::RelativePath::from("/foo%").is_err(), r#"::RelativePath::from("/foo%")"#);
}
#[test]
fn relative_path_from_05() {
    assert!(::RelativePath::from("/foo*").is_err(), r#"::RelativePath::from("/foo*")"#);
}
#[test]
fn relative_path_from_06() {
    assert!(::RelativePath::from("/foo/b:ar").is_err(), r#"::RelativePath::from("/foo/b:ar")"#);
}
#[test]
fn relative_path_from_07() {
    assert!(::RelativePath::from("/foo\"bs").is_err(), r#"::RelativePath::from("/foo\"bs")"#);
}
#[test]
fn relative_path_from_08() {
    assert!(::RelativePath::from("/foo<").is_err(), r#"::RelativePath::from("/foo<")"#);
}
#[test]
fn relative_path_from_09() {
    assert!(::RelativePath::from("/foo>").is_err(), r#"::RelativePath::from("/foo>")"#);
}

#[test]
fn absolute_path_serialize_00() {
    let ser = ::AbsolutePath::from("/foo//bar///qux////").unwrap();
    assert_tokens(&ser, &[Token::String("/foo/bar/qux")]);
}

#[test]
fn absolute_path_serialize_01() {
    let ser = ::AbsolutePath::from("foo//bar///qux////").unwrap();
    assert_tokens(&ser, &[Token::String("/foo/bar/qux")]);
}

#[test]
fn absolute_path_serialize_02() {
    let ser = ::AbsolutePath::from("/").unwrap();
    assert_tokens(&ser, &[Token::String("/")]);
}

#[test]
fn absolute_path_serialize_03() {
    let ser = ::AbsolutePath::from("").unwrap();
    assert_tokens(&ser, &[Token::String("/")]);
}

#[test]
fn absolute_path_append_00() {
    let mut abs = ::AbsolutePath::from("foo").unwrap();
    let rel = ::RelativePath::from("bar").unwrap();
    abs.append(rel);
    assert!(abs == ::AbsolutePath::from("/foo/bar").unwrap())
}
#[test]
fn absolute_path_append_01() {
    let mut abs = ::AbsolutePath::from("/foo//bar///").unwrap();
    let rel = ::RelativePath::from("/qux//do///").unwrap();
    abs.append(rel);
    //println!("{:?}", rel);
    assert!(abs == ::AbsolutePath::from("/foo/bar/qux/do").unwrap())
}

#[test]
fn absolute_path_extend_00() {
    let mut abs = ::AbsolutePath::from("foo").unwrap();
    let rel = ::RelativePath::from("bar").unwrap();
    abs.extend(rel);
    //println!("{:?}", rel);
    assert!(abs == ::AbsolutePath::from("/foo/bar").unwrap())
}
#[test]
fn absolute_path_extend_01() {
    let mut abs = ::AbsolutePath::from("/foo//bar///").unwrap();
    let rel = ::RelativePath::from("/qux//do///").unwrap();
    abs.extend(rel);
    assert!(abs == ::AbsolutePath::from("/foo/bar/qux/do").unwrap())
}