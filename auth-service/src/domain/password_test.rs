#![cfg(test)]

use crate::domain::password::Password;
use crate::domain::errors::PasswordError;

#[test]
fn valid_password_test() {
    let orig = String::from("Abcd1@3456");

    let res = Password::parse(orig.as_ref());
    assert!(res.is_ok(), "expected valid password, got: {:?}", res);

    if let Ok(pwd) = res {
        assert_eq!(pwd.as_ref(), orig.as_str());
    }
}

#[test]
fn invalid_password_length_test() {
    // 9 characters only, should fail
    let res = Password::parse("Abc1@3456".as_ref());
    assert!(matches!(res, Err(PasswordError::BadLength)), "expected BadLength, got: {:?}", res);
}

#[test]
fn invalid_password_no_digit_test() {
    // 10 characters with special but no digit
    let res = Password::parse("Abcd@efghi".as_ref());
    assert!(matches!(res, Err(PasswordError::WeakPassword)), "expected WeakPassword, got: {:?}", res);
}

#[test]
fn invalid_password_no_special_test() {
    // 10 characters with digit but no special character
    let res = Password::parse("Abcd1efghi".as_ref());
    assert!(matches!(res, Err(PasswordError::WeakPassword)), "expected WeakPassword, got: {:?}", res);
}

#[test]
fn valid_password_over_min_length_test() {
    // 11 characters with both digit and special character should be valid
    let orig = String::from("Abcd1@34567");

    let res = Password::parse(orig.as_ref());
    assert!(res.is_ok(), "expected valid long password, got: {:?}", res);

    if let Ok(pwd) = res {
        assert_eq!(pwd.as_ref(), orig.as_str());
    }
}
