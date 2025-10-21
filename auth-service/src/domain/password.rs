use crate::domain::errors::PasswordError;

#[derive(Debug)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self, PasswordError> {
        // Require a minimum length of 10 characters
        if password.chars().count() < 10 {
            return Err(PasswordError::BadLength);
        }

        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        // Consider special characters as ASCII punctuation symbols
        let has_special = password.chars().any(|c| c.is_ascii_punctuation());

        if has_digit && has_special {
            Ok(Self(password))
        } else {
            Err(PasswordError::WeakPassword)
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
