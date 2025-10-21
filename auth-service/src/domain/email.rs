use crate::domain::errors::EmailError;
use std::borrow::Borrow;
use std::hash::{Hash};
use validator::{ValidateEmail};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Email(String);

impl Email {
    pub fn parse(email: &str) -> Result<Self, EmailError> {
        if ValidateEmail::validate_email(&email){
            return Ok(Self(email.to_string()));
        }
        Err(EmailError::InvalidEmail)
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Borrow<str> for Email {
    fn borrow(&self) -> &str {
        &self.0
    }
}
