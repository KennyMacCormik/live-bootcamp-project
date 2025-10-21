#![cfg(test)]

use crate::domain::email::Email;
use crate::domain::errors::EmailError;

#[test]
fn valid_email_test() {
    let orig = String::from("q@b.com");

    let res = Email::parse(orig.clone());
    assert!(res.is_ok(), "expected valid email, got: {:?}", res);

    if let Ok(email) = res {
        assert_eq!(email.as_ref(), orig.as_str());
    }
}

#[test]
fn invalid_email_test() {
    let res = Email::parse("not-an-email".to_string());
    assert!(matches!(res, Err(EmailError::InvalidEmail)), "expected InvalidEmail, got: {:?}", res);
}