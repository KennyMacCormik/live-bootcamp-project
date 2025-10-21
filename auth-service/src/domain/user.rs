use crate::domain::{Email, Password, errors::UserError};

// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
pub struct User {
    email: Email,
    password: Password,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Result<Self, UserError> {
        let email = Email::parse(email.as_ref()).map_err(UserError::EmailParsingError)?;
        let password = Password::parse(password.as_ref()).map_err(UserError::PasswordParsingError)?;
        Ok(User {
            email,
            password,
            requires_2fa,
        })
    }

    pub fn get_email(&self) -> &Email {
        &self.email
    }

    pub fn get_password(&self) -> &Password {
        &self.password
    }
    
    pub fn get_email_as_ref(&self) -> &str {
        &self.email.as_ref()
    }

    pub fn get_password_as_ref(&self) -> &str {
        &self.password.as_ref()
    }
}